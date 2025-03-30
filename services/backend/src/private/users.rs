use poem::web::Data;
use poem_openapi::{OpenApi, payload::Json};

use crate::{auth::User, private::PrivateApiTags};

use super::error::Result;

pub struct Api;

#[OpenApi]
impl Api {
    /// Get the current logged in user.
    #[oai(path = "/users/me", method = "get", tag = PrivateApiTags::User)]
    async fn users_me(&self, Data(user): Data<&User>) -> Result<Json<User>> {
        Ok(Json(user.clone()))
    }
}
