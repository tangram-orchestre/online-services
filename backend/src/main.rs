use std::time::Duration;

use poem::{listener::TcpListener, Route};
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
    let api = OpenApiService::new(Api, "Tangram Orchestre", "1.0.0").url_prefix("/api");
    let docs = api.swagger_ui();
    let spec = api.spec_endpoint();

    let app = Route::new()
        .nest("/api", api)
        .nest("/api/docs", docs)
        .nest("/api/spec", spec);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
