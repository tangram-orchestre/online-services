// @generated automatically by Diesel CLI.

diesel::table! {
    semesters (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        start_date -> Date,
        end_date -> Date,
    }
}
