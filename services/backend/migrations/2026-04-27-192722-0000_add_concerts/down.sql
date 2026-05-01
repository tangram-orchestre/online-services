-- This file should undo anything in `up.sql`

DROP TRIGGER IF EXISTS check_concert_date_in_semester_trigger ON concerts;
DROP FUNCTION IF EXISTS check_concert_date_in_semester();
DROP TABLE concert_pieces;
DROP TABLE pieces;
DROP TABLE concerts;
DROP TABLE locations;
