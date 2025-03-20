use poem::http::HeaderMap;
use poem_openapi::{payload::PlainText, OpenApi, Tags};

pub struct PrivateApi;

#[derive(Tags)]
enum PublicApiTags {
    Placeholder,
}

#[OpenApi]
impl PrivateApi {
    #[oai(path = "/test", method = "get", tag = PublicApiTags::Placeholder)]
    async fn test(&self, headers: &HeaderMap) -> PlainText<String> {
        PlainText(
            headers
                .get("X-App-User")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        )
    }
}
