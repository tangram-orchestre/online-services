use poem_openapi::{payload::PlainText, OpenApi, Tags};

pub struct PrivateApi;

#[derive(Tags)]
enum PublicApiTags {
    Placeholder,
}

#[OpenApi]
impl PrivateApi {
    #[oai(path = "/test", method = "get", tag = PublicApiTags::Placeholder)]
    async fn test(&self) -> PlainText<String> {
        PlainText("Hello, world!".to_string())
    }
}
