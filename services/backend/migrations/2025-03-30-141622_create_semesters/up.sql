-- Your SQL goes here

CREATE TABLE semesters (
  id SERIAL PRIMARY KEY,
  name VARCHAR(32) UNIQUE NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL
  CHECK (start_date < end_date)
);

CREATE OR REPLACE FUNCTION prevent_overlapping_semesters()
RETURNS TRIGGER AS $$
BEGIN
  IF EXISTS (
    SELECT 1
    FROM semesters
    WHERE (NEW.start_date, NEW.end_date) OVERLAPS (start_date, end_date)
      AND id != NEW.id
  ) THEN
    RAISE EXCEPTION check_violation USING MESSAGE = 'Semesters dates cannot overlap with existing semesters';
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_semester_overlap
BEFORE INSERT OR UPDATE ON semesters
FOR EACH ROW
EXECUTE FUNCTION prevent_overlapping_semesters();