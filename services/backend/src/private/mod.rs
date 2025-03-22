use poem::{
    error::{ResponseError, Result},
    http::{HeaderMap, StatusCode},
};
use poem_openapi::{payload::Json, Object, OpenApi, Tags};

pub struct PrivateApi;

#[derive(Tags)]
enum PublicApiTags {
    Placeholder,
}

#[derive(Debug, thiserror::Error)]
#[error("api error")]
struct ApiError;

impl ResponseError for ApiError {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug, Clone, Object)]
struct User {
    id: u32,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    groups: Vec<String>,
}

#[OpenApi]
impl PrivateApi {
    #[oai(path = "/users/me", method = "get", tag = PublicApiTags::Placeholder)]
    async fn users_me(&self, headers: &HeaderMap) -> Result<Json<User>> {
        let get_header = |key: &str| -> Result<String, ApiError> {
            Ok(headers
                .get(key)
                .ok_or(ApiError)?
                .to_str()
                .map_err(|_| ApiError)?
                .to_string())
        };

        let user = User {
            id: get_header("X-App-UserId")?.parse().map_err(|_| ApiError)?,
            username: get_header("X-App-User")?,
            first_name: get_header("X-App-FirstName")?,
            last_name: get_header("X-App-LastName")?,
            email: get_header("X-App-Email")?,
            phone_number: get_header("X-App-PhoneNumber")?,
            groups: get_header("X-App-Groups")?
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        };

        Ok(Json(user))
    }
}
