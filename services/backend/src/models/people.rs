use diesel::prelude::*;
use poem_openapi::Object;

/// Someone of the orchestra credited on pieces. People may have no account.
#[derive(Object, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::people)]
#[diesel(check_for_backend(diesel::pg::Pg))]
// Clearing the display name must set it to NULL, not leave it unchanged.
#[diesel(treat_none_as_null = true)]
pub struct Person {
    pub id: i32,
    #[oai(validator(min_length = 1, max_length = 64))]
    pub first_name: String,
    #[oai(validator(min_length = 1, max_length = 64))]
    pub last_name: String,
    /// Never named on public Tangram material. Excludes a display name.
    pub anonymous: bool,
    /// Alias used on public material instead of the first and last names.
    /// Absent for anonymous people.
    #[oai(validator(min_length = 1, max_length = 128))]
    pub display_name: Option<String>,
}

#[derive(Object, Insertable)]
#[diesel(table_name = crate::schema::people)]
pub struct NewPerson {
    #[oai(validator(min_length = 1, max_length = 64))]
    pub first_name: String,
    #[oai(validator(min_length = 1, max_length = 64))]
    pub last_name: String,
    /// Never named on public Tangram material. Excludes a display name.
    pub anonymous: bool,
    /// Alias used on public material instead of the first and last names.
    /// Absent for anonymous people.
    #[oai(validator(min_length = 1, max_length = 128))]
    pub display_name: Option<String>,
}

/// What a person does on a piece.
#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Copy, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::PieceRole"]
pub enum PieceRole {
    Arranger,
    Conductor,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::piece_people)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PiecePerson {
    pub piece_id: i32,
    pub person_id: i32,
    pub role: PieceRole,
}
