-- Your SQL goes here

-- CREATE TYPE semester_period AS ENUM ('autumn', 'spring');

CREATE TABLE semesters (
  id SERIAL PRIMARY KEY,
  -- period semester_period NOT NULL,
  -- year INTEGER NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL
);
