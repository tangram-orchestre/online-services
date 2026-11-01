-- A semester has a short name (« A26 ») and a long name (« Automne 2026 »).

ALTER TABLE semesters RENAME COLUMN name TO short_name;
ALTER TABLE semesters RENAME CONSTRAINT semesters_name_key TO semesters_short_name_key;

ALTER TABLE semesters ADD COLUMN long_name VARCHAR(64);
UPDATE semesters SET long_name = short_name;
ALTER TABLE semesters
  ALTER COLUMN long_name SET NOT NULL,
  ADD CONSTRAINT semesters_long_name_key UNIQUE (long_name);
