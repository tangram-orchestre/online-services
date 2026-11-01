-- Settings of the website: a single row.
CREATE TABLE settings (
  id BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (id),
  -- Image of concerts without one of their own, of their room or of their
  -- venue. Set by the backend at startup when missing.
  default_concert_image_id UUID REFERENCES images(id) ON DELETE SET NULL
);
INSERT INTO settings DEFAULT VALUES;
