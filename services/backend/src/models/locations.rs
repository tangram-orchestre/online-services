use diesel::prelude::*;
use poem_openapi::Object;

/// A place (an address), with one or more rooms.
#[derive(Object, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
// Clearing the notes must set them to NULL, not leave them unchanged.
#[diesel(treat_none_as_null = true)]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub city: String,
    pub zipcode: String,
    pub street: String,
    /// Practical information: access, keys, piano, parking…
    pub notes: Option<String>,
    /// Photo of the venue, shown for its concerts.
    pub image_id: Option<uuid::Uuid>,
}

#[derive(Object, Insertable)]
#[diesel(table_name = crate::schema::locations)]
pub struct NewLocation {
    pub name: String,
    pub city: String,
    pub zipcode: String,
    pub street: String,
    /// Practical information: access, keys, piano, parking…
    pub notes: Option<String>,
    /// Photo of the venue, shown for its concerts.
    pub image_id: Option<uuid::Uuid>,
}

/// A room of a location, where rehearsals and concerts take place.
///
/// Capacities are optional (unknown). A room without audience capacity can
/// only host rehearsals.
#[derive(Object, Queryable, Selectable)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Room {
    pub id: i32,
    pub name: String,
    /// Number of musicians that fit in the room.
    pub musician_capacity: Option<i32>,
    /// Number of audience seats. Absent for rehearsal-only rooms.
    pub audience_capacity: Option<i32>,
    /// Maximum number of people, musicians and audience together. Cannot be
    /// lower than either capacity, and requires an audience capacity.
    pub total_capacity: Option<i32>,
    /// Photo of the room, shown for its concerts instead of the venue's.
    pub image_id: Option<uuid::Uuid>,
}

/// A room to create (without `id`) or update (with the `id` of one of the
/// location rooms).
#[derive(Object, Insertable, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::rooms)]
// Clearing a capacity must set it to NULL, not leave it unchanged.
#[diesel(treat_none_as_null = true)]
pub struct RoomInput {
    #[diesel(skip_insertion)]
    #[diesel(skip_update)]
    pub id: Option<i32>,
    #[oai(validator(min_length = 1, max_length = 128))]
    pub name: String,
    #[oai(validator(minimum(value = "1")))]
    pub musician_capacity: Option<i32>,
    #[oai(validator(minimum(value = "1")))]
    pub audience_capacity: Option<i32>,
    #[oai(validator(minimum(value = "1")))]
    pub total_capacity: Option<i32>,
    /// Photo of the room, shown for its concerts instead of the venue's.
    pub image_id: Option<uuid::Uuid>,
}

/// Row of `rooms`, as inserted.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::rooms)]
pub struct InsertRoom {
    pub location_id: i32,
    #[diesel(embed)]
    pub room: RoomInput,
}

/// Someone to get in touch with about a location.
#[derive(Object, Queryable, Selectable)]
#[diesel(table_name = crate::schema::location_contacts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LocationContact {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    /// E.g. « Régisseur », « Gardien ».
    pub role: Option<String>,
}

#[derive(Object, Insertable, Clone)]
#[diesel(table_name = crate::schema::location_contacts)]
pub struct NewLocationContact {
    #[oai(validator(min_length = 1, max_length = 128))]
    pub name: String,
    #[oai(validator(max_length = 254, pattern = r"^[^@\s]+@[^@\s]+$"))]
    pub email: Option<String>,
    #[oai(validator(min_length = 1, max_length = 32))]
    pub phone_number: Option<String>,
    #[oai(validator(max_length = 128))]
    pub role: Option<String>,
}

/// Row of `location_contacts`, as inserted.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::location_contacts)]
pub struct InsertLocationContact {
    pub location_id: i32,
    #[diesel(embed)]
    pub contact: NewLocationContact,
}

/// A location along with its rooms and contacts.
#[derive(Object)]
pub struct LocationWithDetails {
    #[oai(flatten)]
    pub location: Location,
    /// Ordered by name.
    pub rooms: Vec<Room>,
    pub contacts: Vec<LocationContact>,
}

/// A location to create, along with its rooms and contacts.
#[derive(Object)]
pub struct NewLocationWithDetails {
    #[oai(flatten)]
    pub location: NewLocation,
    /// Rooms to create; their `id` must be absent.
    pub rooms: Vec<RoomInput>,
    pub contacts: Vec<NewLocationContact>,
}

/// A location to update.
#[derive(Object)]
pub struct LocationUpdate {
    #[oai(flatten)]
    pub location: Location,
    /// Rooms of the location: rooms with an `id` are updated, rooms without
    /// are created, and missing rooms are deleted (which fails if they host
    /// concerts).
    pub rooms: Vec<RoomInput>,
    /// Replace the contacts of the location.
    pub contacts: Vec<NewLocationContact>,
}
