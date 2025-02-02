use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route};
use poem_openapi::{
    payload::{Json, PlainText},
    types::Email,
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
    #[oai(path = "/send_contact_form", method = "post", tag = PublicApiTags::Contact)]
    async fn send_contact_form(&self, contact_form: Json<ContactForm>) -> SendContactFormResponse {
        eprintln!("{:#?}", contact_form);
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
        OpenApiService::new(PublicApi, "Tangram Orchestre Public", "1.0.0").url_prefix("public");
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
