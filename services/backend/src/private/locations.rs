use std::collections::HashMap;

use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{
    AsyncConnection, AsyncPgConnection, RunQueryDsl, scoped_futures::ScopedFutureExt,
};
use poem::web::Data;
use poem_openapi::{OpenApi, param::Path, payload::Json};
use uuid::Uuid;

use crate::auth::policies::{Policy, RequireRole};
use crate::auth::{Principal, Role};
use crate::error::{ApiError, Result};
use crate::images::delete_unused_images;
use crate::{
    AppState,
    models::{
        self, InsertLocationContact, InsertRoom, LocationContact, LocationUpdate,
        LocationWithDetails, NewLocationContact, NewLocationWithDetails, Room, RoomInput,
    },
    private::PrivateApiTags,
    schema::{location_contacts, locations, rooms},
};

pub struct Api;

/// Replaces the contacts of a location.
async fn set_contacts(
    conn: &mut AsyncPgConnection,
    location_id: i32,
    contacts: Vec<NewLocationContact>,
) -> Result<()> {
    diesel::delete(location_contacts::table.filter(location_contacts::location_id.eq(location_id)))
        .execute(conn)
        .await?;
    if !contacts.is_empty() {
        let rows: Vec<InsertLocationContact> = contacts
            .into_iter()
            .map(|contact| InsertLocationContact {
                location_id,
                contact,
            })
            .collect();
        diesel::insert_into(location_contacts::table)
            .values(&rows)
            .execute(conn)
            .await?;
    }
    Ok(())
}

/// Makes the rooms of a location match `rooms`: rooms with an id are updated,
/// rooms without are created, other rooms of the location are deleted.
///
/// Room ids are kept stable since concerts reference them.
async fn set_rooms(
    conn: &mut AsyncPgConnection,
    location_id: i32,
    rooms: Vec<RoomInput>,
) -> Result<()> {
    let kept_ids: Vec<i32> = rooms.iter().filter_map(|r| r.id).collect();
    // Delete first, so that a removed room can be recreated with its name.
    diesel::delete(
        rooms::table
            .filter(rooms::location_id.eq(location_id))
            .filter(rooms::id.ne_all(&kept_ids)),
    )
    .execute(conn)
    .await?;

    let mut new_rooms = vec![];
    for room in rooms {
        match room.id {
            Some(id) => {
                let updated = diesel::update(
                    rooms::table
                        .filter(rooms::id.eq(id))
                        .filter(rooms::location_id.eq(location_id)),
                )
                .set(&room)
                .execute(conn)
                .await?;
                // Not a room of this location.
                if updated == 0 {
                    return Err(ApiError::NotFound);
                }
            }
            None => new_rooms.push(InsertRoom { location_id, room }),
        }
    }
    if !new_rooms.is_empty() {
        diesel::insert_into(rooms::table)
            .values(&new_rooms)
            .execute(conn)
            .await?;
    }
    Ok(())
}

/// Images of a location and of its rooms.
async fn location_images(conn: &mut AsyncPgConnection, location_id: i32) -> Result<Vec<Uuid>> {
    let mut ids: Vec<Uuid> = locations::table
        .find(location_id)
        .select(locations::image_id)
        .load::<Option<Uuid>>(conn)
        .await?
        .into_iter()
        .flatten()
        .collect();
    ids.extend(
        rooms::table
            .filter(rooms::location_id.eq(location_id))
            .select(rooms::image_id)
            .load::<Option<Uuid>>(conn)
            .await?
            .into_iter()
            .flatten(),
    );
    Ok(ids)
}

/// Loads locations, with their rooms and contacts.
async fn load_locations(
    conn: &mut AsyncPgConnection,
    location_id: Option<i32>,
) -> Result<Vec<LocationWithDetails>> {
    let mut locations_query = locations::table
        .select(models::Location::as_select())
        .into_boxed();
    let mut rooms_query = rooms::table
        .order(rooms::name)
        .select((rooms::location_id, Room::as_select()))
        .into_boxed();
    let mut contacts_query = location_contacts::table
        .order(location_contacts::id)
        .select((location_contacts::location_id, LocationContact::as_select()))
        .into_boxed();
    if let Some(id) = location_id {
        locations_query = locations_query.filter(locations::id.eq(id));
        rooms_query = rooms_query.filter(rooms::location_id.eq(id));
        contacts_query = contacts_query.filter(location_contacts::location_id.eq(id));
    }

    let locations = locations_query.load(conn).await?;

    let mut rooms_by_location: HashMap<i32, Vec<Room>> = HashMap::new();
    for (location_id, room) in rooms_query.load::<(i32, Room)>(conn).await? {
        rooms_by_location.entry(location_id).or_default().push(room);
    }
    let mut contacts_by_location: HashMap<i32, Vec<LocationContact>> = HashMap::new();
    for (location_id, contact) in contacts_query.load::<(i32, LocationContact)>(conn).await? {
        contacts_by_location
            .entry(location_id)
            .or_default()
            .push(contact);
    }

    Ok(locations
        .into_iter()
        .map(|location| LocationWithDetails {
            rooms: rooms_by_location.remove(&location.id).unwrap_or_default(),
            contacts: contacts_by_location
                .remove(&location.id)
                .unwrap_or_default(),
            location,
        })
        .collect())
}

/// Loads a single location, with its rooms and contacts.
async fn load_location(
    conn: &mut AsyncPgConnection,
    location_id: i32,
) -> Result<LocationWithDetails> {
    load_locations(conn, Some(location_id))
        .await?
        .pop()
        .ok_or(ApiError::NotFound)
}

#[OpenApi]
impl Api {
    /// Get all locations, with their rooms and contacts
    #[oai(path = "/locations", method = "get", tag = PrivateApiTags::Locations)]
    pub async fn list_locations(
        &self,
        Data(state): Data<&AppState>,
    ) -> Result<Json<Vec<LocationWithDetails>>> {
        let mut conn = state.db_connection_pool.get().await.unwrap();
        // One snapshot for the locations, rooms and contacts queries.
        let locations = conn
            .build_transaction()
            .read_only()
            .repeatable_read()
            .run(|conn| load_locations(conn, None).scope_boxed())
            .await?;
        Ok(Json(locations))
    }

    /// Create a location, with its rooms and contacts
    #[oai(path = "/location", method = "post", tag = PrivateApiTags::Locations)]
    pub async fn create_location(
        &self,
        Data(state): Data<&AppState>,
        Data(principal): Data<&Principal>,
        Json(new_location): Json<NewLocationWithDetails>,
    ) -> Result<Json<LocationWithDetails>> {
        RequireRole(Role::Bureau).check(principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        let created = conn
            .transaction::<_, ApiError, _>(|conn| {
                async move {
                    let location_id = diesel::insert_into(locations::table)
                        .values(&new_location.location)
                        .returning(locations::id)
                        .get_result(conn)
                        .await?;
                    // A new location has no rooms yet: ids are meaningless.
                    let rooms = new_location
                        .rooms
                        .into_iter()
                        .map(|room| RoomInput { id: None, ..room })
                        .collect();
                    set_rooms(conn, location_id, rooms).await?;
                    set_contacts(conn, location_id, new_location.contacts).await?;
                    load_location(conn, location_id).await
                }
                .scope_boxed()
            })
            .await?;
        Ok(Json(created))
    }

    /// Update a location, with its rooms and contacts
    #[oai(path = "/location", method = "put", tag = PrivateApiTags::Locations)]
    pub async fn update_location(
        &self,
        Data(state): Data<&AppState>,
        Json(update): Json<LocationUpdate>,
        principal: Principal,
    ) -> Result<Json<LocationWithDetails>> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        let updated = conn
            .transaction::<_, ApiError, _>(|conn| {
                async move {
                    let location_id = update.location.id;
                    let old_images = location_images(conn, location_id).await?;
                    let found = diesel::update(locations::table.find(location_id))
                        .set(&update.location)
                        .execute(conn)
                        .await?;
                    if found == 0 {
                        return Err(ApiError::NotFound);
                    }
                    set_rooms(conn, location_id, update.rooms).await?;
                    set_contacts(conn, location_id, update.contacts).await?;
                    // Images replaced or of deleted rooms, if not used elsewhere.
                    delete_unused_images(conn, &old_images).await?;
                    load_location(conn, location_id).await
                }
                .scope_boxed()
            })
            .await?;
        Ok(Json(updated))
    }

    /// Delete a location, with its rooms and contacts
    ///
    /// Fails if one of its rooms hosts concerts.
    #[oai(path = "/location/:location_id", method = "delete", tag = PrivateApiTags::Locations)]
    pub async fn delete_location(
        &self,
        Data(state): Data<&AppState>,
        Path(location_id): Path<i32>,
        principal: Principal,
    ) -> Result<()> {
        RequireRole(Role::Bureau).check(&principal)?;
        let mut conn = state.db_connection_pool.get().await.unwrap();
        conn.transaction::<_, ApiError, _>(|conn| {
            async move {
                let images = location_images(conn, location_id).await?;
                // Rooms and contacts are deleted by cascade.
                let deleted = diesel::delete(locations::table.find(location_id))
                    .execute(conn)
                    .await?;
                if deleted == 0 {
                    return Err(ApiError::NotFound);
                }
                delete_unused_images(conn, &images).await?;
                Ok(())
            }
            .scope_boxed()
        })
        .await
    }
}
