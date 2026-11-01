-- Fails if a concert has no room or no start time yet.
ALTER TABLE concerts DROP COLUMN room_booked;
ALTER TABLE concerts
  ALTER COLUMN starts_at SET NOT NULL,
  ALTER COLUMN room_id SET NOT NULL;
