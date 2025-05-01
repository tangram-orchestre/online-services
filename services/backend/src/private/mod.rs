use poem::{
    error::{ResponseError, Result},
    http::{HeaderMap, StatusCode},
    web::Data,
};
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse, Enum, Object, OpenApi, Tags,
};

use diesel::{prelude::*, result::DatabaseErrorKind};
use diesel_async::RunQueryDsl;

use crate::{
    models::{self, NewSemester},
    schema::semesters,
    AppState,
};

pub struct PrivateApi;

#[derive(Tags)]
enum PublicApiTags {
    User,
    Semesters,
}

#[derive(Debug, thiserror::Error)]
#[error("api error")]
struct ApiError;

fn bad_request_handler(err: poem::Error) -> MyError {
    MyError::InternalError(PlainText(format!("error: {}", err.to_string())))
}

#[derive(Enum)]
enum Reason {
    UniqueViolation,
}

#[derive(ApiResponse)]
#[oai(bad_request_handler = "bad_request_handler")]
enum MyError {
    /// Returns when the request parameters is incorrect.
    #[oai(status = 400)]
    BadRequest(Json<Reason>),
    // #[oai(status = 404)]
    // NotFound,
    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

impl From<diesel::result::Error> for MyError {
    fn from(value: diesel::result::Error) -> Self {
        match &value {
            e @ diesel::result::Error::DatabaseError(
                database_error_kind,
                _database_error_information,
            ) => match database_error_kind {
                DatabaseErrorKind::UniqueViolation => {
                    MyError::BadRequest(Json(Reason::UniqueViolation))
                }
                DatabaseErrorKind::CheckViolation => todo!(),
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

impl ResponseError for ApiError {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug, Clone, Object)]
struct User {
    id: u32,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    groups: Vec<String>,
}

#[OpenApi]
impl PrivateApi {
    /// Dummy route, useful as its return type is the same as all other APIs
    #[oai(path = "/dummy", method = "get", tag = PublicApiTags::Semesters)]
    async fn dummy(&self) -> Result<(), MyError> {
        Ok(())
    }

    /// Get the current logged in user.
    #[oai(path = "/users/me", method = "get", tag = PublicApiTags::User)]
    async fn users_me(&self, headers: &HeaderMap) -> Result<Json<User>> {
        let get_header = |key: &str| -> Result<String, ApiError> {
            Ok(headers
                .get(key)
                .ok_or(ApiError)?
                .to_str()
                .map_err(|_| ApiError)?
                .to_string())
        };

        let user = User {
            id: get_header("X-App-UserId")?.parse().map_err(|_| ApiError)?,
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

        Ok(Json(user))
    }

    /// Get the list of all semesters
    #[oai(path = "/semesters", method = "get", tag = PublicApiTags::Semesters)]
    async fn semesters(&self, Data(state): Data<&AppState>) -> Result<Json<Vec<models::Semester>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        let semesters = semesters::table
            .select(models::Semester::as_select())
            .load(&mut conn)
            .await
            .map_err(|_| ApiError)?
            .into_iter()
            .collect();

        Ok(Json(semesters))
    }

    /// Create a new semester
    #[oai(path = "/semester", method = "post", tag = PublicApiTags::Semesters)]
    async fn create_semester(
        &self,
        Data(state): Data<&AppState>,
        new_semester: Json<NewSemester>,
    ) -> Result<Json<models::Semester>, MyError> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        Ok(Json(
            diesel::insert_into(semesters::table)
                .values(&new_semester.0)
                .returning(models::Semester::as_returning())
                .get_result(&mut conn)
                .await?,
        ))
    }

    /// Update a semester
    #[oai(path = "/semester", method = "put", tag = PublicApiTags::Semesters)]
    async fn update_semester(
        &self,
        Data(state): Data<&AppState>,
        Json(semester): Json<models::Semester>,
    ) -> Result<(), MyError> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        diesel::update(semesters::table.find(semester.id))
            .set(&semester)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Delete a semester
    #[oai(path = "/semester/:semester_id", method = "delete", tag = PublicApiTags::Semesters)]
    async fn delete_semester(
        &self,
        Data(state): Data<&AppState>,
        Path(semester_id): Path<i32>,
    ) -> Result<()> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        let deleted = diesel::delete(semesters::table.find(semester_id))
            .execute(&mut conn)
            .await
            .map_err(|_| ApiError)?;

        if deleted == 0 {
            return Err(ApiError.into());
        }

        Ok(())
    }
}
