-- Your SQL goes here

CREATE TABLE semesters (
  id SERIAL PRIMARY KEY,
  name VARCHAR(32) UNIQUE NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL
  CHECK (start_date < end_date)
);
