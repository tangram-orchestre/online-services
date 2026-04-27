use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use poem::web::Data;
use poem_openapi::{OpenApi, param::Path, payload::Json};

use crate::auth::policies::{Policy, RequireRole};
use crate::auth::{Principal, Role};
use crate::error::{ApiError, Result};
use crate::{AppState, models::{self, NewPiece}, private::PrivateApiTags, schema::pieces};

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all pieces
    #[oai(path = "/pieces", method = "get", tag = PrivateApiTags::Pieces)]
    pub async fn list_pieces(
        &self,
        Data(state): Data<&AppState>,
    ) -> Result<Json<Vec<models::Piece>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();
        Ok(Json(
            pieces::table
                .select(models::Piece::as_select())
                .load(&mut conn)
                .await?,
        ))
    }

    /// Create a piece
    #[oai(path = "/piece", method = "post", tag = PrivateApiTags::Pieces)]
    pub async fn create_piece(
        &self,
        Data(state): Data<&AppState>,
        Data(principal): Data<&Principal>,
        Json(new_piece): Json<NewPiece>,
    ) -> Result<Json<models::Piece>> {
        RequireRole(Role::Bureau).check(principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        Ok(Json(
            diesel::insert_into(pieces::table)
                .values(&new_piece)
                .returning(models::Piece::as_returning())
                .get_result(&mut conn)
                .await?,
        ))
    }

    /// Update a piece
    #[oai(path = "/piece", method = "put", tag = PrivateApiTags::Pieces)]
    pub async fn update_piece(
        &self,
        Data(state): Data<&AppState>,
        Json(piece): Json<models::Piece>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        diesel::update(pieces::table.find(piece.id))
            .set(&piece)
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    /// Delete a piece
    #[oai(path = "/piece/:piece_id", method = "delete", tag = PrivateApiTags::Pieces)]
    pub async fn delete_piece(
        &self,
        Data(state): Data<&AppState>,
        Path(piece_id): Path<i32>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        let deleted = diesel::delete(pieces::table.find(piece_id))
            .execute(&mut conn)
            .await?;
        if deleted == 0 {
            return Err(ApiError::NotFound);
        }
        Ok(())
    }
}
