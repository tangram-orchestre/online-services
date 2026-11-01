-- Uploaded images, re-encoded by the backend (never stored as uploaded).
-- They never change: replacing an image creates a new one, so that their URL
-- can be cached forever.
CREATE TABLE images (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  -- image/jpeg or image/png
  content_type VARCHAR(32) NOT NULL,
  width INTEGER NOT NULL,
  height INTEGER NOT NULL,
  data BYTEA NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Photo of the venue, shown for its concerts.
ALTER TABLE locations ADD COLUMN image_id UUID REFERENCES images(id) ON DELETE SET NULL;
CREATE INDEX locations_image_id_idx ON locations (image_id);
-- Photo of a room, shown for its concerts instead of the venue's.
ALTER TABLE rooms ADD COLUMN image_id UUID REFERENCES images(id) ON DELETE SET NULL;
CREATE INDEX rooms_image_id_idx ON rooms (image_id);
-- Replaces the photo of the room and venue (e.g. a poster).
ALTER TABLE concerts ADD COLUMN image_id UUID REFERENCES images(id) ON DELETE SET NULL;
CREATE INDEX concerts_image_id_idx ON concerts (image_id);
