use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use poem::web::Data;
use poem_openapi::{OpenApi, param::Path, payload::Json};

use crate::auth::policies::{Policy, RequireRole};
use crate::auth::{Principal, Role};
use crate::error::{ApiError, Result};
use crate::{AppState, models::{self, NewLocation}, private::PrivateApiTags, schema::locations};

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all locations
    #[oai(path = "/locations", method = "get", tag = PrivateApiTags::Locations)]
    pub async fn list_locations(
        &self,
        Data(state): Data<&AppState>,
    ) -> Result<Json<Vec<models::Location>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();
        Ok(Json(
            locations::table
                .select(models::Location::as_select())
                .load(&mut conn)
                .await?,
        ))
    }

    /// Create a location
    #[oai(path = "/location", method = "post", tag = PrivateApiTags::Locations)]
    pub async fn create_location(
        &self,
        Data(state): Data<&AppState>,
        Data(principal): Data<&Principal>,
        Json(new_location): Json<NewLocation>,
    ) -> Result<Json<models::Location>> {
        RequireRole(Role::Bureau).check(principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        Ok(Json(
            diesel::insert_into(locations::table)
                .values(&new_location)
                .returning(models::Location::as_returning())
                .get_result(&mut conn)
                .await?,
        ))
    }

    /// Update a location
    #[oai(path = "/location", method = "put", tag = PrivateApiTags::Locations)]
    pub async fn update_location(
        &self,
        Data(state): Data<&AppState>,
        Json(location): Json<models::Location>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        diesel::update(locations::table.find(location.id))
            .set(&location)
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    /// Delete a location
    #[oai(path = "/location/:location_id", method = "delete", tag = PrivateApiTags::Locations)]
    pub async fn delete_location(
        &self,
        Data(state): Data<&AppState>,
        Path(location_id): Path<i32>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        let deleted = diesel::delete(locations::table.find(location_id))
            .execute(&mut conn)
            .await?;
        if deleted == 0 {
            return Err(ApiError::NotFound);
        }
        Ok(())
    }
}
