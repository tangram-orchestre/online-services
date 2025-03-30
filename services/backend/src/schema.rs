// @generated automatically by Diesel CLI.

diesel::table! {
    semesters (id) {
        id -> Int4,
        start_date -> Date,
        end_date -> Date,
    }
}
