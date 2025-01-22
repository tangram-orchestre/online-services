use std::time::Duration;

use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route};
use poem_openapi::{param::Query, payload::Json, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> Json<String> {
        tokio::time::sleep(Duration::from_millis(1500)).await;
        match name.0 {
            Some(name) => Json(format!("hello, {}!", name)),
            None => Json("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let public_api =
        OpenApiService::new(Api, "Tangram Orchestre Public", "1.0.0").url_prefix("/public");
    let public_docs = public_api.swagger_ui();
    let public_spec = public_api.spec_endpoint();

    let app = Route::new()
        .nest("/public", public_api)
        .nest("/public/docs", public_docs)
        .nest("/public/spec", public_spec);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        // FIXME: use proper CORS policy
        .run(app.with(Cors::new()))
        .await
}
