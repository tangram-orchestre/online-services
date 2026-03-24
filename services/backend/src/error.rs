use poem_openapi::{
    ApiResponse, Enum, Object, Union,
    payload::{Json, PlainText},
};

use diesel::result::DatabaseErrorKind;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(ApiResponse)]
#[oai(bad_request_handler = "bad_request_handler")]
pub enum ApiError {
    /// Returns when the request parameters is incorrect.
    #[oai(status = 400)]
    BadRequest(Json<BadRequestReason>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    Internal(PlainText<String>),
    #[oai(status = 403)]
    Forbidden,
}

#[derive(Object, Debug, PartialEq)]
pub struct UniqueViolation;

#[derive(Enum, Debug, PartialEq)]
pub enum CheckViolationKind {
    SemestersDateOverlap,
}

#[derive(Object, Debug, PartialEq)]
pub struct CheckViolation {
    kind: Option<CheckViolationKind>,
    message: String,
    details: Option<String>,
}

#[derive(Union, Debug, PartialEq)]
#[oai(discriminator_name = "type")]
pub enum BadRequestReason {
    UniqueViolation(UniqueViolation),
    CheckViolation(CheckViolation),
}

fn bad_request_handler(err: poem::Error) -> ApiError {
    ApiError::Internal(PlainText(err.to_string()))
}

impl ApiError {
    pub(super) fn internal(text: impl Into<String>) -> Self {
        Self::Internal(PlainText(text.into()))
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(value: diesel::result::Error) -> Self {
        match &value {
            e @ diesel::result::Error::DatabaseError(
                database_error_kind,
                database_error_information,
            ) => match database_error_kind {
                DatabaseErrorKind::UniqueViolation => {
                    ApiError::BadRequest(Json(BadRequestReason::UniqueViolation(UniqueViolation)))
                }
                DatabaseErrorKind::CheckViolation => {
                    let kind = match database_error_information.message() {
                        "semesters_date_overlap" => Some(CheckViolationKind::SemestersDateOverlap),
                        other => {
                            tracing::warn!("Unknown check violation: {}", other);
                            None
                        }
                    };

                    ApiError::BadRequest(Json(BadRequestReason::CheckViolation(CheckViolation {
                        kind,
                        message: database_error_information.message().to_string(),
                        details: database_error_information.details().map(|s| s.to_string()),
                    })))
                }
                DatabaseErrorKind::ForeignKeyViolation => todo!(),
                DatabaseErrorKind::NotNullViolation => todo!(),
                DatabaseErrorKind::ClosedConnection
                | DatabaseErrorKind::ReadOnlyTransaction
                | DatabaseErrorKind::SerializationFailure
                | DatabaseErrorKind::UnableToSendCommand => {
                    Self::Internal(PlainText(e.to_string()))
                }
                _ => Self::Internal(PlainText(e.to_string())),
            },
            e => Self::Internal(PlainText(e.to_string())),
        }
    }
}
