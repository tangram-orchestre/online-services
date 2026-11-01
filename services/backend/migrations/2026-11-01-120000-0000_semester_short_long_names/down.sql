ALTER TABLE semesters DROP COLUMN long_name;

ALTER TABLE semesters RENAME CONSTRAINT semesters_short_name_key TO semesters_name_key;
ALTER TABLE semesters RENAME COLUMN short_name TO name;
