use altcha_lib_rs::{verify_solution, Challenge, ChallengeOptions};
use chrono::Utc;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route};
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

#[OpenApi]
impl PublicApi {
    #[oai(path = "/altcha_challenge", method = "get", tag = PublicApiTags::Contact)]
    async fn altcha_challenge(&self) -> Json<Any<Challenge>> {
        let challenge = altcha_lib_rs::create_challenge(ChallengeOptions {
            hmac_key: "super-secret",
            expires: Some(Utc::now() + chrono::TimeDelta::minutes(1)),
            ..Default::default()
        })
        .expect("should be ok");

        Json(Any(challenge))
    }

    #[oai(path = "/send_contact_form", method = "post", tag = PublicApiTags::Contact)]
    async fn send_contact_form(&self, contact_form: Json<ContactForm>) -> SendContactFormResponse {
        // TODO: protect against replay attacks https://github.com/moka-rs/moka
        // TODO: handle error
        // TODO: use secret from conf
        verify_solution(&contact_form.altcha.0, "super-secret", true).expect("should be verified");

        if contact_form.name == "loser" {
            SendContactFormResponse::BadRequest(PlainText(
                "L'envoi du message a échoué.".to_string(),
            ))
        } else {
            SendContactFormResponse::Success
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let public_api =
        OpenApiService::new(PublicApi, "Tangram Orchestre Public", "1.0.0").url_prefix("/public");
    let public_docs = public_api.swagger_ui();
    let public_spec = public_api.spec_endpoint();

    let app = Route::new()
        .nest("/public", public_api)
        .nest("/public/docs", public_docs)
        .nest("/public/spec", public_spec);

    let settings = settings::Settings::load().expect("invalid settings");

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app.with(Cors::new().allow_origins(settings.cors_origins.split(','))))
        .await
}
