use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use altcha_lib_rs::{verify_solution, Challenge, ChallengeOptions};
use chrono::Utc;
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

#[OpenApi]
impl PublicApi {
    #[oai(path = "/altcha_challenge", method = "get", tag = PublicApiTags::Contact)]
    async fn altcha_challenge(&self, state: Data<&Arc<AppState>>) -> Json<Any<Challenge>> {
        let challenge = altcha_lib_rs::create_challenge(ChallengeOptions {
            hmac_key: &state.altcha_secret,
            expires: Some(Utc::now() + chrono::TimeDelta::minutes(ALTCHA_EXPIRATION_MINUTES)),
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

        SendContactFormResponse::Success
    }
}

struct AppState {
    altcha_secret: String,
    altcha_validated_challenges: Mutex<HashMap<String, chrono::DateTime<Utc>>>,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = settings::Settings::load().expect("invalid settings");

    let state = Arc::new(AppState {
        altcha_secret: settings.altcha_secret,
        altcha_validated_challenges: Default::default(),
    });

    let public_api =
        OpenApiService::new(PublicApi, "Tangram Orchestre Public", "1.0.0").url_prefix("/public");
    let public_docs = public_api.swagger_ui();
    let public_spec = public_api.spec_endpoint();

    let app = Route::new()
        .nest("/public", public_api)
        .nest("/public/docs", public_docs)
        .nest("/public/spec", public_spec)
        .with(AddData::new(state));

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app.with(Cors::new().allow_origins(settings.cors_origins.split(','))))
        .await
}
