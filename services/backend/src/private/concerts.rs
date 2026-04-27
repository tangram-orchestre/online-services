use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use poem::web::Data;
use poem_openapi::{OpenApi, param::Path, payload::Json};

use crate::auth::policies::{Policy, RequireRole};
use crate::auth::{Principal, Role};
use crate::error::{ApiError, Result};
use crate::{
    AppState,
    models::{self, ConcertPiece, NewConcert},
    private::PrivateApiTags,
    schema::{concert_pieces, concerts},
};

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all concerts
    #[oai(path = "/concerts", method = "get", tag = PrivateApiTags::Concerts)]
    pub async fn list_concerts(
        &self,
        Data(state): Data<&AppState>,
    ) -> Result<Json<Vec<models::Concert>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();
        Ok(Json(
            concerts::table
                .select(models::Concert::as_select())
                .load(&mut conn)
                .await?,
        ))
    }

    /// Create a concert
    #[oai(path = "/concert", method = "post", tag = PrivateApiTags::Concerts)]
    pub async fn create_concert(
        &self,
        Data(state): Data<&AppState>,
        Data(principal): Data<&Principal>,
        Json(new_concert): Json<NewConcert>,
    ) -> Result<Json<models::Concert>> {
        RequireRole(Role::Bureau).check(principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        Ok(Json(
            diesel::insert_into(concerts::table)
                .values(&new_concert)
                .returning(models::Concert::as_returning())
                .get_result(&mut conn)
                .await?,
        ))
    }

    /// Update a concert
    #[oai(path = "/concert", method = "put", tag = PrivateApiTags::Concerts)]
    pub async fn update_concert(
        &self,
        Data(state): Data<&AppState>,
        Json(concert): Json<models::Concert>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        diesel::update(concerts::table.find(concert.id))
            .set(&concert)
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    /// Delete a concert
    #[oai(path = "/concert/:concert_id", method = "delete", tag = PrivateApiTags::Concerts)]
    pub async fn delete_concert(
        &self,
        Data(state): Data<&AppState>,
        Path(concert_id): Path<i32>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        // Delete join table rows first to avoid FK violation
        diesel::delete(
            concert_pieces::table.filter(concert_pieces::concert_id.eq(concert_id)),
        )
        .execute(&mut conn)
        .await?;
        let deleted = diesel::delete(concerts::table.find(concert_id))
            .execute(&mut conn)
            .await?;
        if deleted == 0 {
            return Err(ApiError::NotFound);
        }
        Ok(())
    }

    /// Add a piece to a concert
    #[oai(path = "/concert/:concert_id/piece/:piece_id", method = "post", tag = PrivateApiTags::Concerts)]
    pub async fn add_piece_to_concert(
        &self,
        Data(state): Data<&AppState>,
        Path(concert_id): Path<i32>,
        Path(piece_id): Path<i32>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        diesel::insert_into(concert_pieces::table)
            .values(ConcertPiece { concert_id, piece_id })
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    /// Remove a piece from a concert
    #[oai(path = "/concert/:concert_id/piece/:piece_id", method = "delete", tag = PrivateApiTags::Concerts)]
    pub async fn remove_piece_from_concert(
        &self,
        Data(state): Data<&AppState>,
        Path(concert_id): Path<i32>,
        Path(piece_id): Path<i32>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        let deleted = diesel::delete(
            concert_pieces::table
                .filter(concert_pieces::concert_id.eq(concert_id))
                .filter(concert_pieces::piece_id.eq(piece_id)),
        )
        .execute(&mut conn)
        .await?;
        if deleted == 0 {
            return Err(ApiError::NotFound);
        }
        Ok(())
    }
}
