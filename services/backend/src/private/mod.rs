use poem::{
    error::{ResponseError, Result},
    http::{HeaderMap, StatusCode},
    web::Data,
};
use poem_openapi::{Object, OpenApi, Tags, param::Path, payload::Json};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    AppState,
    models::{self, NewSemester},
    schema::semesters,
};

pub struct PrivateApi;

#[derive(Tags)]
enum PublicApiTags {
    Placeholder,
}

#[derive(Debug, thiserror::Error)]
#[error("api error")]
struct ApiError;

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

#[derive(Object)]
pub struct SemesterWithName {
    pub id: i32,
    pub name: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

impl From<models::Semester> for SemesterWithName {
    fn from(s: models::Semester) -> Self {
        SemesterWithName {
            id: s.id,
            name: s.name(),
            start_date: s.start_date,
            end_date: s.end_date,
        }
    }
}

#[OpenApi]
impl PrivateApi {
    /// Get the current logged in user.
    #[oai(path = "/users/me", method = "get", tag = PublicApiTags::Placeholder)]
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
    #[oai(path = "/semesters", method = "get", tag = PublicApiTags::Placeholder)]
    async fn semesters(&self, Data(state): Data<&AppState>) -> Result<Json<Vec<SemesterWithName>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        let semesters = semesters::table
            .select(models::Semester::as_select())
            .load(&mut conn)
            .await
            .map_err(|_| ApiError)?
            .into_iter()
            .map(|s| s.into())
            .collect();

        Ok(Json(semesters))
    }

    /// Create a new semester
    #[oai(path = "/semester", method = "post", tag = PublicApiTags::Placeholder)]
    async fn create_semester(
        &self,
        Data(state): Data<&AppState>,
        new_semester: Json<NewSemester>,
    ) -> Result<Json<SemesterWithName>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        Ok(Json(
            diesel::insert_into(semesters::table)
                .values(&new_semester.0)
                .returning(models::Semester::as_returning())
                .get_result(&mut conn)
                .await
                .map_err(|_| ApiError)?
                .into(),
        ))
    }

    /// Update a semester
    #[oai(path = "/semester/:semester_id", method = "put", tag = PublicApiTags::Placeholder)]
    async fn update_semester(
        &self,
        Data(state): Data<&AppState>,
        Path(semester_id): Path<i32>,
        Json(semester): Json<models::NewSemester>,
    ) -> Result<()> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        let updated = diesel::update(semesters::table.find(semester_id))
            .set(&semester)
            .execute(&mut conn)
            .await
            .map_err(|_| ApiError)?;

        if updated == 0 {
            return Err(ApiError.into());
        }

        Ok(())
    }

    /// Delete a semester
    #[oai(path = "/semester/:semester_id", method = "delete", tag = PublicApiTags::Placeholder)]
    async fn delete_semester(
        &self,
        Data(state): Data<&AppState>,
        Path(semester_id): Path<i32>,
    ) -> Result<()> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        diesel::delete(semesters::table.find(semester_id))
            .execute(&mut conn)
            .await
            .map_err(|_| ApiError)?;

        Ok(())
    }
}
