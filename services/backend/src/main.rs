mod private;
mod public;
mod settings;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider};
use tracing_subscriber::{
    EnvFilter,
    filter::{FilterExt, FilterFn},
    prelude::*,
};

use chrono::Utc;
use lettre::{
    AsyncSmtpTransport, Tokio1Executor,
    transport::smtp::{authentication::Credentials, extension::ClientId},
};
use poem::{
    EndpointExt, Route,
    listener::TcpListener,
    middleware::{AddData, Cors},
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

    setup_logging(&settings);

    tracing::info!("Starting server...");

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

    tracing::info!("Listening on port 3000");
    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

fn setup_logging(settings: &settings::Settings) {
    let otlp_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_endpoint(&settings.otlp_endpoint)
        .build()
        .unwrap();
    let provider: SdkLoggerProvider = SdkLoggerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name(settings.otlp_service_name.clone())
                .build(),
        )
        .with_batch_exporter(otlp_exporter)
        .build();

    // To prevent a telemetry-induced-telemetry loop, OpenTelemetry's own internal
    // logging is properly suppressed. However, logs emitted by external components
    // (such as reqwest, tonic, etc.) are not suppressed as they do not propagate
    // OpenTelemetry context. Until this issue is addressed
    // (https://github.com/open-telemetry/opentelemetry-rust/issues/2877),
    // filtering like this is the best way to suppress such logs.
    //
    // The filter levels are set as follows:
    // - Allow `info` level and above by default.
    // - Completely restrict logs from `hyper`, `tonic`, `h2`, and `reqwest`.
    //
    // Note: This filtering will also drop logs from these components even when
    // they are used outside of the OTLP Exporter.
    let filter_otel = EnvFilter::new("info")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("opentelemetry=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap())
        .or(FilterFn::new(|metadata| {
            metadata.target().starts_with("backend") && metadata.level() <= &tracing::Level::DEBUG
        }));
    let otel_layer =
        opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(&provider)
            .with_filter(filter_otel);

    // Create a new tracing::Fmt layer to print the logs to stdout. It has a
    // default filter of `info` level and above, and `debug` and above for logs
    // from OpenTelemetry crates. The filter levels can be customized as needed.
    let filter_fmt = EnvFilter::new("info");
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_filter(filter_fmt);

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(fmt_layer)
        .init();
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
