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

#[derive(Object, Debug, PartialEq)]
pub struct ForeignKeyViolation;

#[derive(Object, Debug, PartialEq)]
pub struct NotNullViolation;

#[derive(Enum, Debug, PartialEq)]
pub enum CheckViolationKind {
    SemestersDateOverlap,
    ConcertDateOutsideSemester,
    ConcertPieceOutsideSemester,
    /// A concert was placed in a room without audience capacity.
    ConcertRoomRehearsalOnly,
    /// The audience capacity of a room hosting concerts was removed.
    RoomHostsConcerts,
    /// A concert room booking was confirmed without a room.
    ConcertRoomBookedWithoutRoom,
    /// The total capacity of a room is lower than one of its capacities.
    RoomTotalCapacityTooSmall,
    /// A total capacity was given to a room without audience.
    RoomTotalCapacityWithoutAudience,
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
    ForeignKeyViolation(ForeignKeyViolation),
    NotNullViolation(NotNullViolation),
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
                    // Errors raised by triggers carry their kind in the
                    // message; table constraints are identified by name.
                    let kind = match database_error_information
                        .constraint_name()
                        .unwrap_or(database_error_information.message())
                    {
                        "semesters_date_overlap" => Some(CheckViolationKind::SemestersDateOverlap),
                        "concert_date_outside_semester" => Some(CheckViolationKind::ConcertDateOutsideSemester),
                        "concert_piece_outside_semester" => {
                            Some(CheckViolationKind::ConcertPieceOutsideSemester)
                        }
                        "concert_room_rehearsal_only" => {
                            Some(CheckViolationKind::ConcertRoomRehearsalOnly)
                        }
                        "room_hosts_concerts" => Some(CheckViolationKind::RoomHostsConcerts),
                        "concerts_room_booked_without_room" => {
                            Some(CheckViolationKind::ConcertRoomBookedWithoutRoom)
                        }
                        "rooms_total_capacity_too_small" => {
                            Some(CheckViolationKind::RoomTotalCapacityTooSmall)
                        }
                        "rooms_total_capacity_without_audience" => {
                            Some(CheckViolationKind::RoomTotalCapacityWithoutAudience)
                        }
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
                DatabaseErrorKind::ForeignKeyViolation => ApiError::BadRequest(Json(
                    BadRequestReason::ForeignKeyViolation(ForeignKeyViolation),
                )),
                DatabaseErrorKind::NotNullViolation => {
                    ApiError::BadRequest(Json(BadRequestReason::NotNullViolation(NotNullViolation)))
                }
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
