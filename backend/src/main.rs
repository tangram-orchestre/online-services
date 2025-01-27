use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route};
use poem_openapi::{payload::Json, types::Email, Object, OpenApi, OpenApiService, Tags};

mod settings;

struct PublicApi;

#[derive(Debug, Clone, Object)]
struct ContactForm {
    /// First name of the sender
    first_name: String,
    /// Last name of the sender
    last_name: String,
    /// Email name of the sender
    email: Email,
    /// Message to be sent
    message: String,
}

#[derive(Tags)]
enum PublicApiTags {
    Contact,
}

#[OpenApi(prefix_path = "/public")]
impl PublicApi {
    #[oai(path = "/send_contact_form", method = "post", tag = PublicApiTags::Contact)]
    async fn send_contact_form(&self, contact_form: Json<ContactForm>) {
        eprintln!("{:#?}", contact_form);
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let public_api = OpenApiService::new(PublicApi, "Tangram Orchestre Public", "1.0.0");
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
