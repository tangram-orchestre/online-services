use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use poem::web::Data;
use poem_openapi::{Multipart, OpenApi, payload::Json, types::multipart::Upload};

use crate::auth::policies::{Policy, RequireRole};
use crate::auth::{Principal, Role};
use crate::error::{ApiError, InvalidImageReason, Result};
use crate::images::{MAX_UPLOAD_BYTES, process_image};
use crate::{AppState, models::ImageMeta, private::PrivateApiTags, schema::images};

#[derive(Multipart)]
pub struct ImageUpload {
    /// JPEG, PNG or WebP, up to 15 MB.
    file: Upload,
}

pub struct Api;

#[OpenApi]
impl Api {
    /// Upload an image
    ///
    /// The image is re-encoded (JPEG, or PNG when transparent), shrunk to fit
    /// 1920×1920 and stripped of its metadata. It is deleted after a day
    /// unless a location or a concert uses it.
    #[oai(path = "/image", method = "post", tag = PrivateApiTags::Images)]
    pub async fn upload_image(
        &self,
        Data(state): Data<&AppState>,
        Data(principal): Data<&Principal>,
        upload: ImageUpload,
    ) -> Result<Json<ImageMeta>> {
        RequireRole(Role::Bureau).check(principal)?;
        if upload.file.size() as u64 > MAX_UPLOAD_BYTES {
            return Err(ApiError::invalid_image(InvalidImageReason::TooLarge));
        }
        let bytes = upload
            .file
            .into_vec()
            .await
            .map_err(|err| ApiError::internal(err.to_string()))?;

        let image = tokio::task::spawn_blocking(move || process_image(&bytes))
            .await
            .map_err(|err| ApiError::internal(err.to_string()))?
            .map_err(|err| {
                tracing::info!("Rejected image upload: {err}");
                ApiError::invalid_image(InvalidImageReason::Unsupported)
            })?;

        let mut conn = state.db_connection_pool.get().await.unwrap();
        Ok(Json(
            diesel::insert_into(images::table)
                .values(&image)
                .returning(ImageMeta::as_returning())
                .get_result(&mut conn)
                .await?,
        ))
    }
}
