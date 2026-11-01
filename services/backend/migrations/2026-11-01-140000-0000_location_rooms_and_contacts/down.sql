DROP TRIGGER check_room_audience_for_concerts_trigger ON rooms;
DROP FUNCTION check_room_audience_for_concerts();
DROP TRIGGER check_concert_room_has_audience_trigger ON concerts;
DROP FUNCTION check_concert_room_has_audience();

ALTER TABLE concerts ADD COLUMN location_id INTEGER REFERENCES locations(id);
UPDATE concerts SET location_id = rooms.location_id
FROM rooms WHERE rooms.id = concerts.room_id;
ALTER TABLE concerts
  ALTER COLUMN location_id SET NOT NULL,
  DROP COLUMN room_id;

DROP TABLE rooms;
DROP TABLE location_contacts;
ALTER TABLE locations DROP COLUMN notes;
