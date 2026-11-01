-- At the start of a semester, concerts are often only a date: the room and
-- the start time come later.
ALTER TABLE concerts
  ALTER COLUMN starts_at DROP NOT NULL,
  ALTER COLUMN room_id DROP NOT NULL;

-- Whether the room booking was confirmed by the venue.
ALTER TABLE concerts
  ADD COLUMN room_booked BOOLEAN NOT NULL DEFAULT FALSE,
  ADD CONSTRAINT concerts_room_booked_without_room
    CHECK (NOT room_booked OR room_id IS NOT NULL);

-- Existing concerts were planned with their room.
UPDATE concerts SET room_booked = TRUE;
