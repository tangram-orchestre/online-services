-- A location (an address) has contacts and one or more rooms. Concerts take
-- place in a room.

-- Practical information: access, keys, piano, parking…
ALTER TABLE locations ADD COLUMN notes TEXT;

CREATE TABLE location_contacts (
  id SERIAL PRIMARY KEY,
  location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
  name VARCHAR(128) NOT NULL,
  email VARCHAR(254),
  phone_number VARCHAR(32),
  role VARCHAR(128)
);
CREATE INDEX location_contacts_location_id_idx ON location_contacts (location_id);

-- Capacities are optional (unknown). A room without audience capacity can
-- only host rehearsals. `total_capacity` caps musicians and audience
-- together, e.g. for fire safety: it cannot be lower than either of them,
-- and makes no sense without an audience.
CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  -- Rooms hosting concerts cannot be deleted (concerts reference them).
  location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
  name VARCHAR(128) NOT NULL,
  musician_capacity INTEGER
    CONSTRAINT rooms_musician_capacity_positive CHECK (musician_capacity > 0),
  audience_capacity INTEGER
    CONSTRAINT rooms_audience_capacity_positive CHECK (audience_capacity > 0),
  total_capacity INTEGER
    CONSTRAINT rooms_total_capacity_positive CHECK (total_capacity > 0),
  CONSTRAINT rooms_total_capacity_too_small CHECK (
    total_capacity >= musician_capacity AND total_capacity >= audience_capacity
  ),
  CONSTRAINT rooms_total_capacity_without_audience CHECK (
    total_capacity IS NULL OR audience_capacity IS NOT NULL
  ),
  CONSTRAINT rooms_name_unique UNIQUE (location_id, name)
);
CREATE INDEX rooms_location_id_idx ON rooms (location_id);

-- Every existing location gets a room, where its concerts move.
INSERT INTO rooms (location_id, name)
SELECT id, 'Salle principale' FROM locations;

ALTER TABLE concerts ADD COLUMN room_id INTEGER REFERENCES rooms(id);
UPDATE concerts SET room_id = rooms.id
FROM rooms WHERE rooms.location_id = concerts.location_id;
ALTER TABLE concerts
  ALTER COLUMN room_id SET NOT NULL,
  DROP COLUMN location_id;
CREATE INDEX concerts_room_id_idx ON concerts (room_id);

-- Concerts need a room with an audience.
CREATE OR REPLACE FUNCTION check_concert_room_has_audience()
RETURNS TRIGGER AS $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM rooms WHERE id = NEW.room_id AND audience_capacity IS NULL
  ) THEN
    RAISE EXCEPTION check_violation USING MESSAGE = 'concert_room_rehearsal_only';
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_concert_room_has_audience_trigger
BEFORE INSERT OR UPDATE OF room_id ON concerts
FOR EACH ROW
EXECUTE FUNCTION check_concert_room_has_audience();

CREATE OR REPLACE FUNCTION check_room_audience_for_concerts()
RETURNS TRIGGER AS $$
BEGIN
  IF NEW.audience_capacity IS NULL
    AND EXISTS (SELECT 1 FROM concerts WHERE room_id = NEW.id)
  THEN
    RAISE EXCEPTION check_violation USING MESSAGE = 'room_hosts_concerts';
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_room_audience_for_concerts_trigger
BEFORE UPDATE OF audience_capacity ON rooms
FOR EACH ROW
EXECUTE FUNCTION check_room_audience_for_concerts();
