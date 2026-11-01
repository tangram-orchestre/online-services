use diesel::prelude::*;
use poem_openapi::Object;

#[derive(Object, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::semesters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Semester {
    pub id: i32,
    /// Short name, e.g. « A26 ».
    #[oai(validator(min_length = 2, max_length = 32))]
    pub short_name: String,
    /// Long name, e.g. « Automne 2026 ».
    #[oai(validator(min_length = 2, max_length = 64))]
    pub long_name: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Object, Insertable)]
#[diesel(table_name = crate::schema::semesters)]
pub struct NewSemester {
    /// Short name, e.g. « A26 ».
    #[oai(validator(min_length = 2, max_length = 32))]
    pub short_name: String,
    /// Long name, e.g. « Automne 2026 ».
    #[oai(validator(min_length = 2, max_length = 64))]
    pub long_name: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}
