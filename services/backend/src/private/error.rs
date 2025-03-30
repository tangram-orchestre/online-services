use poem_openapi::{
    ApiResponse, Object, Union,
    payload::{Json, PlainText},
};

use diesel::result::DatabaseErrorKind;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(ApiResponse)]
#[oai(bad_request_handler = "bad_request_handler")]
pub(super) enum ApiError {
    /// Returns when the request parameters is incorrect.
    #[oai(status = 400)]
    BadRequest(Json<BadRequestReason>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError(PlainText<String>),
    #[oai(status = 403)]
    Forbidden,
}

#[derive(Object, Debug, PartialEq)]
pub(super) struct UniqueViolation;

#[derive(Object, Debug, PartialEq)]
pub(super) struct CheckViolation {
    message: String,
}

#[derive(Union, Debug, PartialEq)]
#[oai(discriminator_name = "type")]
pub(super) enum BadRequestReason {
    UniqueViolation(UniqueViolation),
    CheckViolation(CheckViolation),
}

fn bad_request_handler(err: poem::Error) -> ApiError {
    ApiError::InternalError(PlainText(err.to_string()))
}

impl ApiError {
    // pub(super) fn internal_error(text: impl Into<String>) -> Self {
    //     Self::InternalError(PlainText(text.into()))
    // }
}

impl From<diesel::result::Error> for ApiError {
    fn from(value: diesel::result::Error) -> Self {
        match &value {
            e @ diesel::result::Error::DatabaseError(
                database_error_kind,
                _database_error_information,
            ) => match database_error_kind {
                DatabaseErrorKind::UniqueViolation => {
                    ApiError::BadRequest(Json(BadRequestReason::UniqueViolation(UniqueViolation)))
                }
                DatabaseErrorKind::CheckViolation => {
                    ApiError::BadRequest(Json(BadRequestReason::CheckViolation(CheckViolation {
                        message: e.to_string(),
                    })))
                }
                DatabaseErrorKind::ForeignKeyViolation => todo!(),
                DatabaseErrorKind::NotNullViolation => todo!(),
                DatabaseErrorKind::ClosedConnection
                | DatabaseErrorKind::ReadOnlyTransaction
                | DatabaseErrorKind::SerializationFailure
                | DatabaseErrorKind::UnableToSendCommand => {
                    Self::InternalError(PlainText(e.to_string()))
                }
                _ => Self::InternalError(PlainText(e.to_string())),
            },
            e => Self::InternalError(PlainText(e.to_string())),
        }
    }
}
