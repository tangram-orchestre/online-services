-- This file should undo anything in `up.sql`

DROP TRIGGER check_semester_overlap ON semesters;
DROP FUNCTION prevent_overlapping_semesters;

DROP TABLE semesters;
