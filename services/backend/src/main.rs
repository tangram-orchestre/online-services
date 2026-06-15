mod auth;
mod error;
mod models;
mod private;
mod public;
mod schema;
mod settings;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use diesel::pg::Pg;
use diesel_migrations::{
    EmbeddedMigrations, HarnessWithOutput, MigrationHarness, embed_migrations,
};
use tracing_subscriber::EnvFilter;

use chrono::Utc;
use diesel_async::{
    AsyncConnection, AsyncPgConnection, RunQueryDsl,
    async_connection_wrapper::AsyncConnectionWrapper,
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::{Object, Pool},
    },
};
use lettre::{
    AsyncSmtpTransport, Tokio1Executor,
    transport::smtp::{authentication::Credentials, extension::ClientId},
};
use poem::{
    Endpoint, EndpointExt, IntoResponse, Request, Route,
    listener::TcpListener,
    middleware::{AddData, Cors},
};
use poem_openapi::OpenApiService;
use public::PublicApi;

use crate::error::ApiError;

pub struct AppStateInner {
    altcha_secret: String,
    altcha_validated_challenges: Mutex<HashMap<String, chrono::DateTime<Utc>>>,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    db_connection_pool: Pool<AsyncPgConnection>,
}

impl AppStateInner {
    pub async fn get_db_connection(&self) -> Result<Object<AsyncPgConnection>, ApiError> {
        self.db_connection_pool
            .get()
            .await
            .map_err(|_| ApiError::internal("Failed to get database connection"))
    }
}

pub type AppState = Arc<AppStateInner>;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let public_api =
        OpenApiService::new(PublicApi, "Tangram Orchestre Public", "1.0.0").url_prefix("/public");

    let private_api = OpenApiService::new(private::api(), "Tangram Orchestre Private", "1.0.0");

    let mut should_exit = false;
    if let Ok(path) = std::env::var("PUBLIC_OPENAPI_SPEC_PATH") {
        eprintln!("Writing Public OpenAPI JSON spec to {path}");
        std::fs::write(path, public_api.spec())?;
        should_exit = true;
    }
    if let Ok(path) = std::env::var("PRIVATE_OPENAPI_SPEC_PATH") {
        eprintln!("Writing Private OpenAPI JSON spec to {path}");
        std::fs::write(path, private_api.spec())?;
        should_exit = true;
    }
    if should_exit {
        std::process::exit(0);
    }

    let settings = settings::Settings::load().expect("invalid settings");

    setup_logging();

    tracing::info!("Starting server...");

    let mailer = make_mailer(&settings);

    let db_config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&settings.postgres_url);
    let db_connection_pool = Pool::builder(db_config)
        .build()
        .expect("failed to create connection pool");

    {
        tracing::info!("Running database migrations...");

        let conn = db_connection_pool
            .get()
            .await
            .expect("failed to get connection for migrations");

        run_migrations(conn)
            .await
            .expect("failed to run database migrations");

        tracing::info!("Database migrations complete");
    }

    // If a seed file path is provided, run the seed script.
    if let Some(seed_file_path) = settings.seed_file_path {
        tracing::info!("Running seed script ({})...", seed_file_path);

        let seed_sql = std::fs::read_to_string(seed_file_path).expect("failed to read seed file");

        let mut conn = db_connection_pool
            .get()
            .await
            .expect("failed to get connection for seeding");

        match diesel::sql_query(seed_sql).execute(&mut conn).await {
            Ok(_) => tracing::info!("Seed script complete"),
            Err(err) => tracing::error!("Failed to execute seed SQL: {}", err),
        }
    }

    let state = Arc::new(AppStateInner {
        altcha_secret: settings.altcha_secret,
        altcha_validated_challenges: Default::default(),
        mailer,
        db_connection_pool,
    });

    let public_endpoints = Route::new()
        .nest("/docs", public_api.openapi_explorer())
        .nest("/spec", public_api.spec_endpoint())
        .nest("/", public_api);

    let private_endpoints = Route::new()
        .nest("/docs", private_api.openapi_explorer())
        .nest("/spec", private_api.spec_endpoint())
        .nest("/", private_api.around(auth::require_authentication));

    let app = Route::new()
        .nest("/", private_endpoints)
        .nest("/public", public_endpoints)
        .with(AddData::new(state))
        .with(
            Cors::new()
                .allow_origins(settings.cors_origins.split(','))
                .allow_credentials(true),
        )
        .around(log_errors)
        .around(auth::authenticate);

    tracing::info!("Listening on port 3000");
    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

fn setup_logging() {
    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_env_filter(EnvFilter::new("info"))
        .init();
}

async fn log_errors<E: Endpoint>(
    endpoint: E,
    req: Request,
) -> Result<impl IntoResponse, poem::Error>
where
    E::Output: IntoResponse,
{
    let route = req.uri().clone();
    let method = req.method().clone();
    let principal = req.extensions().get::<auth::Principal>().cloned();

    let res = endpoint.call(req).await;

    let make_error_message = |prefix: &str, err: &dyn std::fmt::Display| {
        format!("{} ({} {}): `{}`", prefix, method, route, err)
    };

    match res {
        Err(err) => {
            tracing::error!("{}", make_error_message("Error processing request", &err));
            Err(err)
        }
        Ok(response) => {
            let response = response.into_response();
            if !response.status().is_success() {
                tracing::warn!(
                    { principal = ?principal },
                    "{}",
                    make_error_message(
                        "Request resulted in an error response",
                        &format!("{:?}", response.status())
                    )
                );
            }

            Ok(response)
        }
    }
}

fn make_mailer(settings: &settings::Settings) -> AsyncSmtpTransport<Tokio1Executor> {
    match (&settings.smtp_name, &settings.smtp_password) {
        (Some(name), Some(password)) => {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&settings.smtp_host)
                .unwrap()
                .credentials(Credentials::new(name.clone(), password.clone()))
                .hello_name(ClientId::Domain(settings.host.to_string()))
        }
        (None, None) => {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(settings.smtp_host.clone())
                .port(1025)
        }
        _ => panic!("SMTP name and password must be both set or both unset"),
    }
    .build()
}

async fn run_migrations<A>(async_connection: A) -> Result<(), Box<dyn std::error::Error>>
where
    A: AsyncConnection<Backend = Pg> + 'static,
{
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    let mut async_wrapper: AsyncConnectionWrapper<A> =
        AsyncConnectionWrapper::from(async_connection);

    tokio::task::spawn_blocking(move || {
        let mut harness = HarnessWithOutput::write_to_stdout(&mut async_wrapper);
        harness.run_pending_migrations(MIGRATIONS).unwrap();
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Ok(())
}
