//! Uploaded images: processing and cleanup.
//!
//! An upload is never stored as is: it is decoded (so it must really be an
//! image), oriented, shrunk and re-encoded, which also strips its metadata
//! (EXIF: GPS coordinates, camera…) before it can reach the public website.

use std::io::Cursor;
use std::time::Duration;

use diesel::sql_types::Array;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use image::{
    DynamicImage, ImageDecoder, ImageFormat, ImageReader, Limits, codecs::jpeg::JpegEncoder,
    imageops::FilterType,
};
use uuid::Uuid;

use crate::AppState;
use crate::models::NewImage;
use crate::schema::{images, locations, rooms, settings};

/// Largest accepted upload.
pub const MAX_UPLOAD_BYTES: u64 = 15 * 1024 * 1024;
/// Images are shrunk to fit in a square of this size.
const MAX_DIMENSION: u32 = 1920;
const JPEG_QUALITY: u8 = 85;
/// Unused images are kept this long, for forms not saved yet.
const UNUSED_IMAGE_GRACE: &str = "24 hours";

#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    #[error("unsupported or invalid image: {0}")]
    Invalid(#[from] image::ImageError),
    #[error("could not detect the image format: {0}")]
    UnknownFormat(#[from] std::io::Error),
}

/// Decode `bytes`, orient and shrink the image, then re-encode it as JPEG, or
/// as PNG when it has transparent pixels.
///
/// CPU bound: to run with `spawn_blocking`.
pub fn process_image(bytes: &[u8]) -> Result<NewImage, ImageError> {
    let mut reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
    // Against decompression bombs: a small file declaring huge dimensions.
    let mut limits = Limits::default();
    limits.max_image_width = Some(12_000);
    limits.max_image_height = Some(12_000);
    limits.max_alloc = Some(512 * 1024 * 1024);
    reader.limits(limits);

    let mut decoder = reader.into_decoder()?;
    let orientation = decoder.orientation()?;
    let mut image = DynamicImage::from_decoder(decoder)?;
    image.apply_orientation(orientation);

    if image.width() > MAX_DIMENSION || image.height() > MAX_DIMENSION {
        image = image.resize(MAX_DIMENSION, MAX_DIMENSION, FilterType::Lanczos3);
    }

    let transparent = image.color().has_alpha() && image.to_rgba8().pixels().any(|p| p[3] < 255);

    let mut data = Vec::new();
    let content_type = if transparent {
        image.write_to(&mut Cursor::new(&mut data), ImageFormat::Png)?;
        "image/png"
    } else {
        JpegEncoder::new_with_quality(&mut data, JPEG_QUALITY).encode_image(&image.to_rgb8())?;
        "image/jpeg"
    };

    Ok(NewImage {
        content_type: content_type.to_string(),
        width: image.width() as i32,
        height: image.height() as i32,
        data,
    })
}

/// Condition on `images` rows that no location, room, concert or setting uses.
const UNUSED: &str = "NOT EXISTS (SELECT 1 FROM locations WHERE locations.image_id = images.id) \
     AND NOT EXISTS (SELECT 1 FROM rooms WHERE rooms.image_id = images.id) \
     AND NOT EXISTS (SELECT 1 FROM concerts WHERE concerts.image_id = images.id) \
     AND NOT EXISTS (SELECT 1 FROM settings WHERE settings.default_concert_image_id = images.id)";

/// Delete the images of `ids` that no location, room or concert uses.
///
/// Called when an owner drops or replaces its image, in the same transaction.
pub async fn delete_unused_images(
    conn: &mut AsyncPgConnection,
    ids: &[Uuid],
) -> diesel::QueryResult<usize> {
    if ids.is_empty() {
        return Ok(0);
    }
    diesel::sql_query(format!(
        "DELETE FROM images WHERE id = ANY($1) AND {UNUSED}"
    ))
    .bind::<Array<diesel::sql_types::Uuid>, _>(ids)
    .execute(conn)
    .await
}

/// Image of concerts until another one is chosen in the portal.
const DEFAULT_CONCERT_IMAGE: &[u8] = include_bytes!("../assets/default-concert.jpg");

/// Make sure there is a default concert image, creating it from the bundled
/// one when missing (first start, or the setting was cleared).
pub async fn ensure_default_concert_image(
    conn: &mut AsyncPgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let current: Option<Uuid> = settings::table
        .select(settings::default_concert_image_id)
        .first(conn)
        .await?;
    if current.is_some() {
        return Ok(());
    }
    let image = tokio::task::spawn_blocking(|| process_image(DEFAULT_CONCERT_IMAGE)).await??;
    let id: Uuid = diesel::insert_into(images::table)
        .values(&image)
        .returning(images::id)
        .get_result(conn)
        .await?;
    diesel::update(settings::table)
        .set(settings::default_concert_image_id.eq(id))
        .execute(conn)
        .await?;
    tracing::info!("Created the default concert image");
    Ok(())
}

/// Photos of the seeded venues and rooms (location, room, file in
/// `seeds/images`), for development and staging data.
const SEED_IMAGES: &[(&str, Option<&str>, &str)] = &[
    ("Salle Gaveau", None, "salle-gaveau.jpg"),
    (
        "Conservatoire du 11e",
        Some("Auditorium"),
        "conservatoire-auditorium.jpg",
    ),
];

/// Give their photo to the seeded venues and rooms that have none, from the
/// files of `dir`.
pub async fn seed_images(
    conn: &mut AsyncPgConnection,
    dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    for &(location, room, file) in SEED_IMAGES {
        // The location or room, if it exists and has no image.
        let target = match room {
            None => locations::table
                .filter(locations::name.eq(location))
                .filter(locations::image_id.is_null())
                .select(locations::id)
                .first::<i32>(conn)
                .await
                .optional()?,
            Some(room) => rooms::table
                .inner_join(locations::table)
                .filter(locations::name.eq(location))
                .filter(rooms::name.eq(room))
                .filter(rooms::image_id.is_null())
                .select(rooms::id)
                .first::<i32>(conn)
                .await
                .optional()?,
        };
        let Some(target) = target else { continue };

        let bytes = std::fs::read(dir.join(file))?;
        let image = tokio::task::spawn_blocking(move || process_image(&bytes)).await??;
        let id: Uuid = diesel::insert_into(images::table)
            .values(&image)
            .returning(images::id)
            .get_result(conn)
            .await?;
        if room.is_some() {
            diesel::update(rooms::table.find(target))
                .set(rooms::image_id.eq(id))
                .execute(conn)
                .await?;
        } else {
            diesel::update(locations::table.find(target))
                .set(locations::image_id.eq(id))
                .execute(conn)
                .await?;
        }
        tracing::info!(
            "Seeded the photo of {location} {}",
            room.unwrap_or_default()
        );
    }
    Ok(())
}

/// Delete, now and then every hour, the unused images older than a day:
/// uploads of forms that were never saved.
pub fn spawn_unused_images_cleanup(state: AppState) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60 * 60));
        loop {
            interval.tick().await;
            let Ok(mut conn) = state.get_db_connection().await else {
                tracing::warn!("Unused images cleanup: no database connection");
                continue;
            };
            let deleted = diesel::sql_query(format!(
                "DELETE FROM images WHERE created_at < now() - interval '{UNUSED_IMAGE_GRACE}' \
                 AND {UNUSED}"
            ))
            .execute(&mut conn)
            .await;
            match deleted {
                Ok(0) => {}
                Ok(n) => tracing::info!("Deleted {n} unused images"),
                Err(err) => tracing::error!("Unused images cleanup failed: {err}"),
            }
        }
    });
}
