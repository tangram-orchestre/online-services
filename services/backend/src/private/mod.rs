use poem::{get, http::HeaderMap};
use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi, Tags,
};

pub struct PrivateApi;

#[derive(Tags)]
enum PublicApiTags {
    Placeholder,
}

#[derive(Debug, Clone, Object)]
struct User {
    id: u32,
    username: String,
    first_name: String,
    last_name: String,
    groups: Vec<String>,
}

#[OpenApi]
impl PrivateApi {
    #[oai(path = "/users/me", method = "get", tag = PublicApiTags::Placeholder)]
    async fn users_me(&self, headers: &HeaderMap) -> Json<User> {
        let get_header = |key: &str| headers.get(key).unwrap().to_str().unwrap().to_string();

        dbg!(headers);

        let user = User {
            id: get_header("X-App-UserId").parse().unwrap(),
            username: get_header("X-App-User"),
            first_name: get_header("X-App-FirstName"),
            last_name: get_header("X-App-LastName"),
            groups: get_header("X-App-Groups")
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        };

        Json(user)
    }
}
