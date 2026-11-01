// @generated automatically by Diesel CLI.

diesel::table! {
    concert_pieces (concert_id, piece_id) {
        concert_id -> Int4,
        piece_id -> Int4,
        position -> Int4,
    }
}

diesel::table! {
    concerts (id) {
        id -> Int4,
        date -> Date,
        tickets_url -> Nullable<Text>,
        video_url -> Nullable<Text>,
        doors_open_at -> Nullable<Time>,
        meeting_at -> Nullable<Time>,
        starts_at -> Nullable<Time>,
        public -> Bool,
        semester_id -> Int4,
        room_id -> Nullable<Int4>,
        room_booked -> Bool,
        #[max_length = 128]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    location_contacts (id) {
        id -> Int4,
        location_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 254]
        email -> Nullable<Varchar>,
        #[max_length = 32]
        phone_number -> Nullable<Varchar>,
        #[max_length = 128]
        role -> Nullable<Varchar>,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        city -> Varchar,
        #[max_length = 16]
        zipcode -> Varchar,
        #[max_length = 256]
        street -> Varchar,
        notes -> Nullable<Text>,
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
    rooms (id) {
        id -> Int4,
        location_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        musician_capacity -> Nullable<Int4>,
        audience_capacity -> Nullable<Int4>,
        total_capacity -> Nullable<Int4>,
    }
}

diesel::table! {
    semesters (id) {
        id -> Int4,
        #[max_length = 32]
        short_name -> Varchar,
        start_date -> Date,
        end_date -> Date,
        #[max_length = 64]
        long_name -> Varchar,
    }
}

diesel::joinable!(concert_pieces -> concerts (concert_id));
diesel::joinable!(concert_pieces -> pieces (piece_id));
diesel::joinable!(concerts -> rooms (room_id));
diesel::joinable!(concerts -> semesters (semester_id));
diesel::joinable!(location_contacts -> locations (location_id));
diesel::joinable!(pieces -> semesters (semester_id));
diesel::joinable!(rooms -> locations (location_id));

diesel::allow_tables_to_appear_in_same_query!(
    concert_pieces,
    concerts,
    location_contacts,
    locations,
    pieces,
    rooms,
    semesters,
);
