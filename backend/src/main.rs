use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use altcha_lib_rs::{verify_solution, Challenge, ChallengeOptions};
use chrono::Utc;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use poem::{
    listener::TcpListener,
    middleware::{AddData, Cors},
    web::Data,
    EndpointExt, Route,
};
use poem_openapi::{
    payload::{Json, PlainText},
    types::{Any, Email},
    ApiResponse, Object, OpenApi, OpenApiService, Tags,
};

mod settings;

struct PublicApi;

#[derive(Debug, Clone, Object)]
struct ContactForm {
    /// Name of the sender
    #[oai(validator { min_length = 2,  max_length = 50 })]
    name: String,
    /// Email name of the sender
    #[oai(validator { max_length = 50 })]
    email: Email,
    /// Subject of the message
    #[oai(validator { min_length = 2,  max_length = 50 })]
    subject: String,
    /// Message to be sent
    #[oai(validator { min_length = 3, max_length = 2000 })]
    message: String,
    /// Altcha captcha
    altcha: Any<altcha_lib_rs::Payload>,
}

#[derive(Tags)]
enum PublicApiTags {
    Contact,
}

#[derive(ApiResponse)]
enum SendContactFormResponse {
    #[oai(status = 200)]
    Success,
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

const ALTCHA_EXPIRATION_MINUTES: i64 = 1;
// See https://altcha.org/docs/complexity/
const ALTCHA_CHALLENGE_COMPLEXITY: u64 = 100_000;

#[OpenApi]
impl PublicApi {
    #[oai(path = "/altcha_challenge", method = "get", tag = PublicApiTags::Contact)]
    async fn altcha_challenge(&self, state: Data<&Arc<AppState>>) -> Json<Any<Challenge>> {
        let challenge = altcha_lib_rs::create_challenge(ChallengeOptions {
            hmac_key: &state.altcha_secret,
            expires: Some(Utc::now() + chrono::TimeDelta::minutes(ALTCHA_EXPIRATION_MINUTES)),
            max_number: Some(ALTCHA_CHALLENGE_COMPLEXITY),
            ..Default::default()
        })
        .expect("should be ok");

        eprintln!("Altcha challenge created: {:?}", challenge);

        Json(Any(challenge))
    }

    #[oai(path = "/send_contact_form", method = "post", tag = PublicApiTags::Contact)]
    async fn send_contact_form(
        &self,
        contact_form: Json<ContactForm>,
        state: Data<&Arc<AppState>>,
    ) -> SendContactFormResponse {
        if let Err(e) = verify_solution(&contact_form.altcha.0, &state.altcha_secret, true) {
            eprintln!("Altcha challenge could not be validated: {:?}", e);
            return SendContactFormResponse::BadRequest(PlainText(
                "Altcha challenge could not be validated".to_string(),
            ));
        }

        // Protect against replay attacks
        {
            let mut validated_challenges = state.altcha_validated_challenges.lock().unwrap();
            if validated_challenges.contains_key(&contact_form.altcha.0.salt) {
                eprintln!(
                    "Altcha challenge already validated {:?}",
                    contact_form.email
                );
                return SendContactFormResponse::BadRequest(PlainText(
                    "Altcha challenge already validated".to_string(),
                ));
            }

            // Remove expired challenges
            let now = Utc::now();
            validated_challenges.retain(|_, datetime| {
                now - *datetime > chrono::TimeDelta::minutes(ALTCHA_EXPIRATION_MINUTES)
            });

            validated_challenges.insert(contact_form.altcha.0.salt.clone(), now);
        }

        let email = match Message::builder()
            .from(
                "NoReply Tangram <noreply@tangram-orchestre.fr>"
                    .parse()
                    .unwrap(),
            )
            .reply_to(Mailbox::new(
                Some(contact_form.name.clone()),
                contact_form.email.0.parse().unwrap(),
            ))
            .to("Contact Tangram <contact@tangram-orchestre.fr>"
                .parse()
                .unwrap())
            .subject(format!(
                "[Website Contact Form] {} - {}",
                contact_form.name, contact_form.subject
            ))
            .header(ContentType::TEXT_PLAIN)
            .body(contact_form.message.clone())
        {
            Ok(email) => email,
            e => {
                eprintln!("Could not create email: {:?}", e);
                return SendContactFormResponse::BadRequest(PlainText(
                    "Could not create email".to_string(),
                ));
            }
        };

        // Send the email
        match state.mailer.send(email).await {
            Ok(_) => {
                eprintln!(
                    "Email sent (sender: {:?}), alcha time: {:?}ms",
                    contact_form.email,
                    contact_form.altcha.0.took.unwrap_or(0)
                );
                SendContactFormResponse::Success
            }
            Err(_e) => {
                eprintln!("Could not send email: {:?}", _e);
                SendContactFormResponse::BadRequest(PlainText("Could not send email".to_string()))
            }
        }
    }
}

struct AppState {
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

    eprintln!("Starting");
    if let Ok(path) = std::env::var("PUBLIC_OPENAPI_SPEC_PATH") {
        eprintln!("Writing Public OpenAPI spec to {}", path);
        std::fs::write(path, public_api.spec())?;

        std::process::exit(0);
    }

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
        .with(AddData::new(state))
        .with(
            Cors::new()
                .allow_origins(settings.cors_origins.split(','))
                .allow_credentials(true),
        );

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

fn make_mailer(settings: &settings::Settings) -> AsyncSmtpTransport<Tokio1Executor> {
    match (&settings.smtp_name, &settings.smtp_password) {
        (Some(name), Some(password)) => {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&settings.smtp_host)
                .unwrap()
                .credentials(Credentials::new(name.clone(), password.clone()))
        }
        (None, None) => {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(settings.smtp_host.clone())
                .port(1025)
        }
        _ => panic!("SMTP name and password must be both set or both unset"),
    }
    .build()
}
