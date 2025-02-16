use poem::{listener::TcpListener, Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api = OpenApiService::new(Api, "Tangram Orchestre", "1.0.0").server("/api");
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
