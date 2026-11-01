import type { LocationWithDetails, Room } from "#hey-api/types.gen";

export type Place = { location: LocationWithDetails; room: Room };

/** Locations, and where each room is. */
export const useLocations = () => {
  const {
    data: locations,
    refresh,
    status,
  } = getLocations({
    composable: "useAsyncData",
    key: "getLocations",
  });

  const places = computed(() => {
    const byRoom = new Map<number, Place>();
    for (const location of locations.value ?? []) {
      for (const room of location.rooms) {
        byRoom.set(room.id, { location, room });
      }
    }
    return byRoom;
  });

  /** Location and room of a room id. */
  const place = (roomId?: number | null) =>
    roomId == null ? undefined : places.value.get(roomId);

  /** « Salle Gaveau · Grande salle » */
  const placeName = (roomId?: number | null) => {
    const p = place(roomId);
    return p ? `${p.location.name} · ${p.room.name}` : undefined;
  };

  return { locations, refresh, status, place, placeName };
};

/** A room without audience capacity only hosts rehearsals. */
export const isRehearsalOnly = (room: Room) => room.audience_capacity == null;
