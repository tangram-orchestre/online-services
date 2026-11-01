use diesel::prelude::*;
use poem_openapi::Object;

/// An uploaded image, without its data.
#[derive(Object, Queryable, Selectable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImageMeta {
    pub id: uuid::Uuid,
    /// `image/jpeg` or `image/png`.
    pub content_type: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::images)]
pub struct NewImage {
    pub content_type: String,
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}
