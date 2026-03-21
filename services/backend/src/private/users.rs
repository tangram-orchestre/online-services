use poem::web::Data;
use poem_openapi::{OpenApi, payload::Json};

use crate::auth::Principal;
use crate::{auth::User, private::PrivateApiTags};

use crate::error::Result;

pub struct Api;

#[OpenApi]
impl Api {
    /// Get the current logged in user.
    #[oai(path = "/users/me", method = "get", tag = PrivateApiTags::User)]
    async fn users_me(&self, Data(principal): Data<&Principal>) -> Result<Json<User>> {
        Ok(Json(principal.user.clone()))
    }
}
