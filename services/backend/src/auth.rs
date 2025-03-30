use poem::{
    Endpoint, IntoResponse, Request, Response, Result,
    http::{HeaderMap, StatusCode},
};
use poem_openapi::Object;

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

fn extract_user_from_headers(headers: &HeaderMap) -> Option<User> {
    let get_header =
        |key: &str| -> Option<String> { Some(headers.get(key)?.to_str().ok()?.to_string()) };

    let user = User {
        id: get_header("X-App-UserId")?.parse().ok()?,
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

    Some(user)
}

pub(crate) async fn authenticate<E: Endpoint>(next: E, mut req: Request) -> Result<Response> {
    let Some(user) = extract_user_from_headers(req.headers()) else {
        return Err(poem::Error::from_status(StatusCode::UNAUTHORIZED));
    };

    req.extensions_mut().insert(user);

    next.call(req).await.map(|res| res.into_response())
}
