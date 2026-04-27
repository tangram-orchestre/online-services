use diesel::prelude::*;
use poem_openapi::Object;

#[derive(Object, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Location {
    pub id: i32,
    pub city: String,
    pub zipcode: String,
    pub street: String,
}

#[derive(Object, Insertable)]
#[diesel(table_name = crate::schema::locations)]
pub struct NewLocation {
    pub city: String,
    pub zipcode: String,
    pub street: String,
}

#[derive(Object, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::concerts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Concert {
    pub id: i32,
    pub date: chrono::NaiveDate,
    pub tickets_url: Option<String>,
    pub video_url: Option<String>,
    pub doors_open_at: Option<chrono::NaiveTime>,
    pub starts_at: chrono::NaiveTime,
    pub public: bool,
    pub location_id: i32,
}

#[derive(Object, Insertable)]
#[diesel(table_name = crate::schema::concerts)]
pub struct NewConcert {
    pub date: chrono::NaiveDate,
    pub tickets_url: Option<String>,
    pub video_url: Option<String>,
    pub doors_open_at: Option<chrono::NaiveTime>,
    pub starts_at: chrono::NaiveTime,
    pub public: bool,
    pub location_id: i32,
}

#[derive(Object, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::pieces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Piece {
    pub id: i32,
    pub name: String,
    pub source: Option<String>,
    pub composer: Option<String>,
    pub arranger: Option<String>,
}

#[derive(Object, Insertable)]
#[diesel(table_name = crate::schema::pieces)]
pub struct NewPiece {
    pub name: String,
    pub source: Option<String>,
    pub composer: Option<String>,
    pub arranger: Option<String>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::concert_pieces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ConcertPiece {
    pub concert_id: i32,
    pub piece_id: i32,
}
