use std::collections::HashMap;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::{
    AsyncConnection, AsyncPgConnection, RunQueryDsl, scoped_futures::ScopedFutureExt,
};
use poem::web::Data;
use poem_openapi::{OpenApi, param::Path, payload::Json};

use crate::auth::policies::{Policy, RequireRole};
use crate::auth::{Principal, Role};
use crate::error::{ApiError, Result};
use crate::{
    AppState,
    models::{self, NewPieceWithPeople, PiecePerson, PieceRole, PieceWithPeople},
    private::PrivateApiTags,
    schema::{piece_people, pieces},
};

/// Replace the people credited on a piece.
async fn set_piece_people(
    conn: &mut AsyncPgConnection,
    piece_id: i32,
    arranger_ids: &[i32],
    conductor_ids: &[i32],
) -> Result<()> {
    diesel::delete(piece_people::table.filter(piece_people::piece_id.eq(piece_id)))
        .execute(conn)
        .await?;

    let mut links: Vec<PiecePerson> = Vec::new();
    for (role, ids) in [
        (PieceRole::Arranger, arranger_ids),
        (PieceRole::Conductor, conductor_ids),
    ] {
        for &person_id in ids {
            // Listing someone twice is harmless.
            if !links
                .iter()
                .any(|l| l.role == role && l.person_id == person_id)
            {
                links.push(PiecePerson {
                    piece_id,
                    person_id,
                    role,
                });
            }
        }
    }
    if !links.is_empty() {
        diesel::insert_into(piece_people::table)
            .values(&links)
            .execute(conn)
            .await?;
    }
    Ok(())
}

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all pieces, with the people credited on them
    #[oai(path = "/pieces", method = "get", tag = PrivateApiTags::Pieces)]
    pub async fn list_pieces(
        &self,
        Data(state): Data<&AppState>,
    ) -> Result<Json<Vec<PieceWithPeople>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();
        // One snapshot for both queries, so that credits match pieces.
        let (pieces, links) = conn
            .build_transaction()
            .read_only()
            .repeatable_read()
            .run::<_, ApiError, _>(|conn| {
                async move {
                    let pieces = pieces::table
                        .select(models::Piece::as_select())
                        .load(conn)
                        .await?;
                    let links = piece_people::table
                        .order((piece_people::piece_id, piece_people::person_id))
                        .select(PiecePerson::as_select())
                        .load(conn)
                        .await?;
                    Ok((pieces, links))
                }
                .scope_boxed()
            })
            .await?;

        let mut links_by_piece: HashMap<i32, Vec<PiecePerson>> = HashMap::new();
        for link in links {
            links_by_piece.entry(link.piece_id).or_default().push(link);
        }

        Ok(Json(
            pieces
                .into_iter()
                .map(|piece| {
                    let links = links_by_piece.remove(&piece.id).unwrap_or_default();
                    let ids = |role| {
                        links
                            .iter()
                            .filter(|l| l.role == role)
                            .map(|l| l.person_id)
                            .collect()
                    };
                    PieceWithPeople {
                        arranger_ids: ids(PieceRole::Arranger),
                        conductor_ids: ids(PieceRole::Conductor),
                        piece,
                    }
                })
                .collect(),
        ))
    }

    /// Create a piece
    #[oai(path = "/piece", method = "post", tag = PrivateApiTags::Pieces)]
    pub async fn create_piece(
        &self,
        Data(state): Data<&AppState>,
        Data(principal): Data<&Principal>,
        Json(new_piece): Json<NewPieceWithPeople>,
    ) -> Result<Json<PieceWithPeople>> {
        RequireRole(Role::Bureau).check(principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        let piece = conn
            .transaction::<_, ApiError, _>(|conn| {
                async move {
                    let piece = diesel::insert_into(pieces::table)
                        .values(&new_piece.piece)
                        .returning(models::Piece::as_returning())
                        .get_result(conn)
                        .await?;
                    set_piece_people(
                        conn,
                        piece.id,
                        &new_piece.arranger_ids,
                        &new_piece.conductor_ids,
                    )
                    .await?;
                    Ok(PieceWithPeople {
                        piece,
                        arranger_ids: new_piece.arranger_ids,
                        conductor_ids: new_piece.conductor_ids,
                    })
                }
                .scope_boxed()
            })
            .await?;
        Ok(Json(piece))
    }

    /// Update a piece
    ///
    /// Replaces the people credited on the piece with the given ones.
    #[oai(path = "/piece", method = "put", tag = PrivateApiTags::Pieces)]
    pub async fn update_piece(
        &self,
        Data(state): Data<&AppState>,
        Json(piece): Json<PieceWithPeople>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        conn.transaction::<_, ApiError, _>(|conn| {
            async move {
                diesel::update(pieces::table.find(piece.piece.id))
                    .set(&piece.piece)
                    .returning(pieces::id)
                    .get_result::<i32>(conn)
                    .await
                    .optional()?
                    .ok_or(ApiError::NotFound)?;
                set_piece_people(
                    conn,
                    piece.piece.id,
                    &piece.arranger_ids,
                    &piece.conductor_ids,
                )
                .await
            }
            .scope_boxed()
        })
        .await
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
        // The credits are deleted by cascade.
        let deleted = diesel::delete(pieces::table.find(piece_id))
            .execute(&mut conn)
            .await?;
        if deleted == 0 {
            return Err(ApiError::NotFound);
        }
        Ok(())
    }
}
