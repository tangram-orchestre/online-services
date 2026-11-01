use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncConnection, RunQueryDsl, scoped_futures::ScopedFutureExt};
use poem::web::Data;
use poem_openapi::{Object, OpenApi, payload::Json};
use uuid::Uuid;

use crate::auth::policies::{Policy, RequireRole};
use crate::auth::{Principal, Role};
use crate::error::{ApiError, Result};
use crate::images::delete_unused_images;
use crate::{AppState, private::PrivateApiTags, schema::settings};

/// Settings of the website.
#[derive(Object)]
pub struct Settings {
    /// Image of concerts without one of their own, of their room or of their
    /// venue. Cannot be removed, only replaced.
    default_concert_image_id: Uuid,
}

pub struct Api;

#[OpenApi]
impl Api {
    /// Get the settings of the website
    #[oai(path = "/settings", method = "get", tag = PrivateApiTags::Settings)]
    pub async fn get_settings(&self, Data(state): Data<&AppState>) -> Result<Json<Settings>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();
        let default_concert_image_id: Option<Uuid> = settings::table
            .select(settings::default_concert_image_id)
            .first(&mut conn)
            .await?;
        Ok(Json(Settings {
            // Created at startup.
            default_concert_image_id: default_concert_image_id
                .ok_or_else(|| ApiError::internal("No default concert image"))?,
        }))
    }

    /// Update the settings of the website
    #[oai(path = "/settings", method = "put", tag = PrivateApiTags::Settings)]
    pub async fn update_settings(
        &self,
        Data(state): Data<&AppState>,
        Json(new_settings): Json<Settings>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        conn.transaction::<_, ApiError, _>(|conn| {
            async move {
                let old: Option<Uuid> = settings::table
                    .select(settings::default_concert_image_id)
                    .first(conn)
                    .await?;
                diesel::update(settings::table)
                    .set(
                        settings::default_concert_image_id
                            .eq(new_settings.default_concert_image_id),
                    )
                    .execute(conn)
                    .await?;
                // The replaced default, unless used elsewhere.
                delete_unused_images(conn, old.as_slice()).await?;
                Ok(())
            }
            .scope_boxed()
        })
        .await
    }
}
