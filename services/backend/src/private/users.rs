use poem::http::HeaderMap;
use poem_openapi::{Object, OpenApi, payload::Json};

use crate::private::PrivateApiTags;

use super::error::{ApiError, Result};

pub struct Api;

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
impl Api {
    /// Get the current logged in user.
    #[oai(path = "/users/me", method = "get", tag = PrivateApiTags::User)]
    async fn users_me(&self, headers: &HeaderMap) -> Result<Json<User>> {
        let get_header = |key: &str| -> Result<String> {
            Ok(headers
                .get(key)
                .ok_or_else(|| ApiError::internal_error(format!("Missing `{key}` header")))?
                .to_str()
                .map_err(|e| {
                    ApiError::internal_error(format!("Could not convert `{key}` to str: {e}"))
                })?
                .to_string())
        };

        let user = User {
            id: get_header("X-App-UserId")?.parse().map_err(|e| {
                ApiError::internal_error(format!("Could not convert UserId to u32: {e}"))
            })?,
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
