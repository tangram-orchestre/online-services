use poem::{Endpoint, IntoResponse, Request, Response, Result, http::StatusCode};
use poem_openapi::Object;

pub mod policies;
mod principal;

pub use principal::{Principal, Role};

use crate::auth::principal::extract_principal_from_headers;

#[derive(Debug, Clone, Object)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub groups: Vec<String>,
}

pub(crate) async fn authenticate<E: Endpoint>(next: E, mut req: Request) -> Result<Response> {
    if let Some(principal) = extract_principal_from_headers(req.headers()) {
        req.extensions_mut().insert(principal);
    }

    next.call(req).await.map(|res| res.into_response())
}

pub(crate) async fn require_authentication<E: Endpoint>(next: E, req: Request) -> Result<Response> {
    if req.extensions().get::<Principal>().is_none() {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized"));
    }

    next.call(req).await.map(|res| res.into_response())
}
