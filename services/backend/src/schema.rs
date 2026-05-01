// @generated automatically by Diesel CLI.

diesel::table! {
    concert_pieces (concert_id, piece_id) {
        concert_id -> Int4,
        piece_id -> Int4,
    }
}

diesel::table! {
    concerts (id) {
        id -> Int4,
        date -> Date,
        tickets_url -> Nullable<Text>,
        video_url -> Nullable<Text>,
        doors_open_at -> Nullable<Time>,
        starts_at -> Time,
        public -> Bool,
        location_id -> Int4,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        #[max_length = 128]
        city -> Varchar,
        #[max_length = 16]
        zipcode -> Varchar,
        #[max_length = 256]
        street -> Varchar,
    }
}

diesel::table! {
    pieces (id) {
        id -> Int4,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 256]
        source -> Nullable<Varchar>,
        #[max_length = 128]
        composer -> Nullable<Varchar>,
        #[max_length = 128]
        arranger -> Nullable<Varchar>,
        semester_id -> Int4,
    }
}

diesel::table! {
    semesters (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        start_date -> Date,
        end_date -> Date,
    }
}

diesel::joinable!(concert_pieces -> concerts (concert_id));
diesel::joinable!(concert_pieces -> pieces (piece_id));
diesel::joinable!(concerts -> locations (location_id));
diesel::joinable!(pieces -> semesters (semester_id));

diesel::allow_tables_to_appear_in_same_query!(
    concert_pieces,
    concerts,
    locations,
    pieces,
    semesters,
);
