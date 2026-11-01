-- People of the orchestra credited on pieces (arrangement, direction). They
-- may have no account, so they are not tied to users.
CREATE TABLE people (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR(64) NOT NULL,
  last_name VARCHAR(64) NOT NULL,
  -- Never named on public Tangram material (showcase website, programmes…).
  anonymous BOOLEAN NOT NULL DEFAULT FALSE,
  -- Alias used on public material instead of the first and last names.
  display_name VARCHAR(128),
  -- Someone anonymous is not named at all, not even by an alias.
  CONSTRAINT people_anonymous_with_display_name CHECK (
    NOT (anonymous AND display_name IS NOT NULL)
  )
);

CREATE TYPE piece_role AS ENUM ('arranger', 'conductor');

CREATE TABLE piece_people (
  piece_id INTEGER NOT NULL REFERENCES pieces(id) ON DELETE CASCADE,
  -- A person credited on a piece cannot be deleted.
  person_id INTEGER NOT NULL REFERENCES people(id),
  role piece_role NOT NULL,
  PRIMARY KEY (piece_id, role, person_id)
);
CREATE INDEX piece_people_person_id_idx ON piece_people (person_id);

-- Arrangers are now people.
ALTER TABLE pieces DROP COLUMN arranger;
