use poem_openapi::OpenApi;

use crate::private::PrivateApiTags;

use super::error::Result;

pub(super) struct Api;

#[OpenApi]
impl Api {
    /// Dummy route
    ///
    /// Useful as its return type is the same as all other APIs
    #[oai(path = "/dummy", method = "get", tag = PrivateApiTags::Misc)]
    async fn dummy(&self) -> Result<()> {
        Ok(())
    }
}
