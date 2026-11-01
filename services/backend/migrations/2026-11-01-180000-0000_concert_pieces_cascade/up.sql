-- A programme only exists with its concert. Pieces still cannot be deleted
-- while a concert plays them.
ALTER TABLE concert_pieces
  DROP CONSTRAINT concert_pieces_concert_id_fkey,
  ADD CONSTRAINT concert_pieces_concert_id_fkey
    FOREIGN KEY (concert_id) REFERENCES concerts(id) ON DELETE CASCADE;
