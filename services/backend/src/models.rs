use diesel::prelude::*;
use poem_openapi::Object;

#[derive(Object, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::semesters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Semester {
    pub id: i32,
    #[oai(validator(min_length = 2))]
    pub name: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Object, Insertable)]
#[diesel(table_name = crate::schema::semesters)]
pub struct NewSemester {
    #[oai(validator(min_length = 2))]
    pub name: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}
