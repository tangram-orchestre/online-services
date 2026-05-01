-- Your SQL goes here

CREATE TABLE locations (
  id SERIAL PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  city VARCHAR(128) NOT NULL,
  zipcode VARCHAR(16) NOT NULL,
  street VARCHAR(256) NOT NULL
);

CREATE TABLE concerts (
  id SERIAL PRIMARY KEY,
  date DATE NOT NULL,
  tickets_url TEXT,
  video_url TEXT,
  doors_open_at TIME,
  starts_at TIME NOT NULL,
  public BOOLEAN NOT NULL DEFAULT FALSE,
  location_id INTEGER NOT NULL REFERENCES locations(id),
  semester_id INTEGER NOT NULL REFERENCES semesters(id),
  CHECK (doors_open_at IS NULL OR doors_open_at < starts_at)
);

CREATE TABLE pieces (
  id SERIAL PRIMARY KEY,
  name VARCHAR(256) NOT NULL,
  source VARCHAR(256),
  composer VARCHAR(128),
  arranger VARCHAR(128),
  semester_id INTEGER NOT NULL REFERENCES semesters(id)
);

CREATE TABLE concert_pieces (
  concert_id INTEGER NOT NULL REFERENCES concerts(id),
  piece_id INTEGER NOT NULL REFERENCES pieces(id),
  PRIMARY KEY (concert_id, piece_id)
);


CREATE OR REPLACE FUNCTION check_concert_date_in_semester()
RETURNS TRIGGER AS $$
BEGIN
  IF NOT EXISTS (
    SELECT 1 FROM semesters
    WHERE id = NEW.semester_id
      AND NEW.date >= start_date
      AND NEW.date <= end_date
  ) THEN
    RAISE EXCEPTION check_violation USING MESSAGE = 'concert_date_outside_semester';
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_concert_date_in_semester_trigger
BEFORE INSERT OR UPDATE ON concerts
FOR EACH ROW
EXECUTE FUNCTION check_concert_date_in_semester();
