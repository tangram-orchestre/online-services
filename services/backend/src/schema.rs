// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "piece_role"))]
    pub struct PieceRole;
}

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
        image_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    images (id) {
        id -> Uuid,
        #[max_length = 32]
        content_type -> Varchar,
        width -> Int4,
        height -> Int4,
        data -> Bytea,
        created_at -> Timestamptz,
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
        image_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    people (id) {
        id -> Int4,
        #[max_length = 64]
        first_name -> Varchar,
        #[max_length = 64]
        last_name -> Varchar,
        anonymous -> Bool,
        #[max_length = 128]
        display_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PieceRole;

    piece_people (piece_id, role, person_id) {
        piece_id -> Int4,
        person_id -> Int4,
        role -> PieceRole,
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
        image_id -> Nullable<Uuid>,
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

diesel::table! {
    settings (id) {
        id -> Bool,
        default_concert_image_id -> Nullable<Uuid>,
    }
}

diesel::joinable!(concert_pieces -> concerts (concert_id));
diesel::joinable!(concert_pieces -> pieces (piece_id));
diesel::joinable!(concerts -> images (image_id));
diesel::joinable!(concerts -> rooms (room_id));
diesel::joinable!(concerts -> semesters (semester_id));
diesel::joinable!(location_contacts -> locations (location_id));
diesel::joinable!(locations -> images (image_id));
diesel::joinable!(piece_people -> people (person_id));
diesel::joinable!(piece_people -> pieces (piece_id));
diesel::joinable!(pieces -> semesters (semester_id));
diesel::joinable!(rooms -> images (image_id));
diesel::joinable!(rooms -> locations (location_id));
diesel::joinable!(settings -> images (default_concert_image_id));

diesel::allow_tables_to_appear_in_same_query!(
    concert_pieces,
    concerts,
    images,
    location_contacts,
    locations,
    people,
    piece_people,
    pieces,
    rooms,
    semesters,
    settings,
);
