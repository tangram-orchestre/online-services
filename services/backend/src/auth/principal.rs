use std::sync::Arc;

use poem::{FromRequest, http::HeaderMap};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Role {
    Orga,
    Devs,
    Bureau,
}

/// The `InnerPrincipal` struct represents the authenticated user and their associated information.
#[derive(Clone)]
pub struct InnerPrincipal {
    pub user: super::User,
}

impl std::fmt::Debug for InnerPrincipal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Principal {{ username: {}, id: {}, groups: {:?} }}",
            self.user.username, self.user.id, self.user.groups
        )
    }
}

impl InnerPrincipal {
    /// Checks if the user has a specific role based on their group memberships.
    pub fn has_role(&self, role: Role) -> bool {
        match role {
            Role::Orga => self.user.groups.contains(&"Orga".to_string()),
            Role::Devs => self.user.groups.contains(&"Devs".to_string()),
            Role::Bureau => self.user.groups.contains(&"Bureau".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct Principal(Arc<InnerPrincipal>);

impl std::fmt::Debug for Principal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::ops::Deref for Principal {
    type Target = InnerPrincipal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest<'_> for Principal {
    async fn from_request(
        req: &poem::Request,
        _body: &mut poem::RequestBody,
    ) -> poem::Result<Self> {
        req.extensions().get::<Principal>().cloned().ok_or_else(|| {
            poem::Error::from_string(
                "Unauthorized".to_string(),
                poem::http::StatusCode::UNAUTHORIZED,
            )
        })
    }
}

pub(super) fn extract_principal_from_headers(headers: &HeaderMap) -> Option<Principal> {
    let get_header =
        |key: &str| -> Option<String> { Some(headers.get(key)?.to_str().ok()?.to_string()) };

    let user = super::User {
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

    Some(Principal(Arc::new(InnerPrincipal { user })))
}
