mod private;
mod public;
mod settings;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use chrono::Utc;
use lettre::{
    transport::smtp::{authentication::Credentials, extension::ClientId},
    AsyncSmtpTransport, Tokio1Executor,
};
use poem::{
    listener::TcpListener,
    middleware::{AddData, Cors},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use private::PrivateApi;
use public::PublicApi;

pub(crate) struct AppState {
    altcha_secret: String,
    altcha_validated_challenges: Mutex<HashMap<String, chrono::DateTime<Utc>>>,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let public_api =
        OpenApiService::new(PublicApi, "Tangram Orchestre Public", "1.0.0").url_prefix("/public");
    let public_docs = public_api.swagger_ui();
    let public_spec = public_api.spec_endpoint();

    let private_api = OpenApiService::new(PrivateApi, "Tangram Orchestre Private", "1.0.0");
    let private_docs = private_api.swagger_ui();
    let private_spec = private_api.spec_endpoint();

    let mut should_exit = false;
    if let Ok(path) = std::env::var("PUBLIC_OPENAPI_SPEC_PATH") {
        eprintln!("Writing Public OpenAPI spec to {}", path);
        std::fs::write(path, public_api.spec())?;
        should_exit = true;
    }
    if let Ok(path) = std::env::var("PRIVATE_OPENAPI_JSON_PATH") {
        eprintln!("Writing Private OpenAPI JSON to {}", path);
        std::fs::write(path, public_api.spec())?;
        should_exit = true;
    }
    if should_exit {
        std::process::exit(0);
    }

    eprintln!("Starting server...");
    let settings = settings::Settings::load().expect("invalid settings");

    let mailer = make_mailer(&settings);

    let state = Arc::new(AppState {
        altcha_secret: settings.altcha_secret,
        altcha_validated_challenges: Default::default(),
        mailer,
    });

    let app = Route::new()
        .nest("/public", public_api)
        .nest("/public/docs", public_docs)
        .nest("/public/spec", public_spec)
        .nest("/", private_api)
        .nest("/docs", private_docs)
        .nest("/spec", private_spec)
        .with(AddData::new(state))
        .with(
            Cors::new()
                .allow_origins(settings.cors_origins.split(','))
                .allow_credentials(true),
        );

    eprintln!("Listening on port 3000");
    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
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
