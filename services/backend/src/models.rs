use chrono::Datelike;
use diesel::prelude::*;
use poem_openapi::Object;

#[derive(Object, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::semesters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Semester {
    pub id: i32,
    pub name: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

impl Semester {
    pub fn name(&self) -> String {
        let year = self.start_date.year() - 2000;
        let semester = if self.start_date.month0() < 6 {
            'P'
        } else {
            'A'
        };
        format!("{semester}{year}")
    }
}

#[derive(Object, Insertable)]
#[diesel(table_name = crate::schema::semesters)]
pub struct NewSemester {
    pub name: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}
