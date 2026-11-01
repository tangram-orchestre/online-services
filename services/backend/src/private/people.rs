use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use poem::web::Data;
use poem_openapi::{OpenApi, param::Path, payload::Json};

use crate::auth::policies::{Policy, RequireRole};
use crate::auth::{Principal, Role};
use crate::error::{ApiError, Result};
use crate::{
    AppState,
    models::{self, NewPerson},
    private::PrivateApiTags,
    schema::people,
};

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all people
    #[oai(path = "/people", method = "get", tag = PrivateApiTags::People)]
    pub async fn list_people(
        &self,
        Data(state): Data<&AppState>,
    ) -> Result<Json<Vec<models::Person>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();
        Ok(Json(
            people::table
                .select(models::Person::as_select())
                .load(&mut conn)
                .await?,
        ))
    }

    /// Create a person
    #[oai(path = "/person", method = "post", tag = PrivateApiTags::People)]
    pub async fn create_person(
        &self,
        Data(state): Data<&AppState>,
        Data(principal): Data<&Principal>,
        Json(new_person): Json<NewPerson>,
    ) -> Result<Json<models::Person>> {
        RequireRole(Role::Bureau).check(principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        Ok(Json(
            diesel::insert_into(people::table)
                .values(&new_person)
                .returning(models::Person::as_returning())
                .get_result(&mut conn)
                .await?,
        ))
    }

    /// Update a person
    #[oai(path = "/person", method = "put", tag = PrivateApiTags::People)]
    pub async fn update_person(
        &self,
        Data(state): Data<&AppState>,
        Json(person): Json<models::Person>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        let updated = diesel::update(people::table.find(person.id))
            .set(&person)
            .execute(&mut conn)
            .await?;
        if updated == 0 {
            return Err(ApiError::NotFound);
        }
        Ok(())
    }

    /// Delete a person
    ///
    /// Fails while the person is credited on a piece.
    #[oai(path = "/person/:person_id", method = "delete", tag = PrivateApiTags::People)]
    pub async fn delete_person(
        &self,
        Data(state): Data<&AppState>,
        Path(person_id): Path<i32>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        let deleted = diesel::delete(people::table.find(person_id))
            .execute(&mut conn)
            .await?;
        if deleted == 0 {
            return Err(ApiError::NotFound);
        }
        Ok(())
    }
}
