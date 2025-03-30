use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use poem_openapi::{OpenApi, param::Path};

use crate::{
    AppState,
    auth::User,
    models::{self, NewSemester},
    private::{
        PrivateApiTags,
        error::{ApiError, Result},
    },
    schema::semesters,
};

use poem_openapi::payload::Json;

use poem::web::Data;

pub struct Api;

#[OpenApi]
impl Api {
    /// Get the list of all semesters
    #[oai(path = "/semesters", method = "get", tag = PrivateApiTags::Semesters)]
    pub(crate) async fn semesters(
        &self,
        Data(state): Data<&AppState>,
    ) -> Result<Json<Vec<models::Semester>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        let semesters = semesters::table
            .select(models::Semester::as_select())
            .load(&mut conn)
            .await?
            .into_iter()
            .collect();

        Ok(Json(semesters))
    }

    /// Create a new semester
    #[oai(path = "/semester", method = "post", tag = PrivateApiTags::Semesters)]
    pub(crate) async fn create_semester(
        &self,
        Data(state): Data<&AppState>,
        Data(user): Data<&User>,
        new_semester: Json<NewSemester>,
    ) -> Result<Json<models::Semester>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        if !user.groups.contains(&"Orga".to_string()) {
            return Err(ApiError::Forbidden);
        }

        Ok(Json(
            diesel::insert_into(semesters::table)
                .values(&new_semester.0)
                .returning(models::Semester::as_returning())
                .get_result(&mut conn)
                .await?,
        ))
    }

    /// Update a semester
    #[oai(path = "/semester", method = "put", tag = PrivateApiTags::Semesters)]
    pub(crate) async fn update_semester(
        &self,
        Data(state): Data<&AppState>,
        Json(semester): Json<models::Semester>,
    ) -> Result<()> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        diesel::update(semesters::table.find(semester.id))
            .set(&semester)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Delete a semester
    #[oai(path = "/semester/:semester_id", method = "delete", tag = PrivateApiTags::Semesters)]
    pub(crate) async fn delete_semester(
        &self,
        Data(state): Data<&AppState>,
        Path(semester_id): Path<i32>,
    ) -> Result<()> {
        let mut conn = state.db_connection_pool.get().await.unwrap();

        let deleted = diesel::delete(semesters::table.find(semester_id))
            .execute(&mut conn)
            .await?;

        if deleted == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }
}
