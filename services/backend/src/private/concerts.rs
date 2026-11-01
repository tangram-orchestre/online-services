use std::collections::HashMap;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::{AsyncConnection, RunQueryDsl, scoped_futures::ScopedFutureExt};
use poem::web::Data;
use poem_openapi::{OpenApi, param::Path, payload::Json};

use crate::auth::policies::{Policy, RequireRole};
use crate::auth::{Principal, Role};
use crate::error::{ApiError, Result};
use crate::{
    AppState,
    models::{self, ConcertPiece, ConcertWithPieces, NewConcert},
    private::PrivateApiTags,
    schema::{concert_pieces, concerts},
};

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all concerts, with their programme
    #[oai(path = "/concerts", method = "get", tag = PrivateApiTags::Concerts)]
    pub async fn list_concerts(
        &self,
        Data(state): Data<&AppState>,
    ) -> Result<Json<Vec<ConcertWithPieces>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();
        // One snapshot for both queries, so that programmes match concerts.
        let (concerts, links) = conn
            .build_transaction()
            .read_only()
            .repeatable_read()
            .run::<_, ApiError, _>(|conn| {
                async move {
                    let concerts = concerts::table
                        .select(models::Concert::as_select())
                        .load(conn)
                        .await?;
                    let links = concert_pieces::table
                        .order((concert_pieces::concert_id, concert_pieces::position))
                        .select(ConcertPiece::as_select())
                        .load(conn)
                        .await?;
                    Ok((concerts, links))
                }
                .scope_boxed()
            })
            .await?;

        let mut piece_ids_by_concert: HashMap<i32, Vec<i32>> = HashMap::new();
        for link in links {
            piece_ids_by_concert
                .entry(link.concert_id)
                .or_default()
                .push(link.piece_id);
        }

        Ok(Json(
            concerts
                .into_iter()
                .map(|concert| ConcertWithPieces {
                    piece_ids: piece_ids_by_concert.remove(&concert.id).unwrap_or_default(),
                    concert,
                })
                .collect(),
        ))
    }

    /// Create a concert
    #[oai(path = "/concert", method = "post", tag = PrivateApiTags::Concerts)]
    pub async fn create_concert(
        &self,
        Data(state): Data<&AppState>,
        Data(principal): Data<&Principal>,
        Json(new_concert): Json<NewConcert>,
    ) -> Result<Json<ConcertWithPieces>> {
        RequireRole(Role::Bureau).check(principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        let concert = diesel::insert_into(concerts::table)
            .values(&new_concert)
            .returning(models::Concert::as_returning())
            .get_result(&mut conn)
            .await?;
        Ok(Json(ConcertWithPieces {
            concert,
            piece_ids: vec![],
        }))
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
        // The programme is deleted by cascade.
        let deleted = diesel::delete(concerts::table.find(concert_id))
            .execute(&mut conn)
            .await?;
        if deleted == 0 {
            return Err(ApiError::NotFound);
        }
        Ok(())
    }

    /// Set the programme of a concert
    ///
    /// Replaces the concert pieces with the given ones, in playing order.
    #[oai(path = "/concert/:concert_id/pieces", method = "put", tag = PrivateApiTags::Concerts)]
    pub async fn set_concert_pieces(
        &self,
        Data(state): Data<&AppState>,
        Path(concert_id): Path<i32>,
        Json(piece_ids): Json<Vec<i32>>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        conn.transaction::<_, ApiError, _>(|conn| {
            async move {
                concerts::table
                    .find(concert_id)
                    .select(concerts::id)
                    .first::<i32>(conn)
                    .await
                    .optional()?
                    .ok_or(ApiError::NotFound)?;

                diesel::delete(
                    concert_pieces::table.filter(concert_pieces::concert_id.eq(concert_id)),
                )
                .execute(conn)
                .await?;

                let links: Vec<ConcertPiece> = piece_ids
                    .iter()
                    .enumerate()
                    .map(|(position, &piece_id)| ConcertPiece {
                        concert_id,
                        piece_id,
                        position: position as i32,
                    })
                    .collect();
                if !links.is_empty() {
                    diesel::insert_into(concert_pieces::table)
                        .values(&links)
                        .execute(conn)
                        .await?;
                }
                Ok(())
            }
            .scope_boxed()
        })
        .await
    }
}
