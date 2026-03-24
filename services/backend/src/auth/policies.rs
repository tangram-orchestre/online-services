use crate::{
    auth::{Principal, Role},
    error::ApiError,
};

#[allow(dead_code)]
pub trait Policy {
    fn check(&self, principal: &Principal) -> std::result::Result<(), ApiError> {
        match self.check_impl(principal) {
            Ok(()) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn check_impl(&self, principal: &Principal) -> std::result::Result<(), ApiError>;

    fn and<P: Policy>(self, other: P) -> impl Policy
    where
        Self: Sized,
    {
        struct And<L: Policy, R: Policy> {
            left: L,
            right: R,
        }

        impl<L: Policy, R: Policy> Policy for And<L, R>
        where
            L: Policy,
            R: Policy,
        {
            fn check_impl(&self, principal: &Principal) -> std::result::Result<(), ApiError> {
                self.left.check_impl(principal)?;
                self.right.check_impl(principal)
            }
        }

        And {
            left: self,
            right: other,
        }
    }

    fn or<P: Policy>(self, other: P) -> impl Policy
    where
        Self: Sized,
    {
        struct Or<L: Policy, R: Policy> {
            left: L,
            right: R,
        }

        impl<L: Policy, R: Policy> Policy for Or<L, R>
        where
            L: Policy,
            R: Policy,
        {
            fn check_impl(&self, principal: &Principal) -> std::result::Result<(), ApiError> {
                match self.left.check_impl(principal) {
                    Ok(()) => Ok(()),
                    Err(_) => self.right.check_impl(principal),
                }
            }
        }

        Or {
            left: self,
            right: other,
        }
    }
}

pub struct RequireRole(pub Role);

impl Policy for RequireRole {
    fn check_impl(&self, principal: &Principal) -> std::result::Result<(), ApiError> {
        if principal.has_role(self.0) {
            Ok(())
        } else {
            Err(ApiError::Forbidden)
        }
    }
}
