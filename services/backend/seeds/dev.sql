-- Development data. Runs at every backend start, so each part only inserts
-- what is missing.
--
-- Semesters are computed from the current date: the seed always provides the
-- semester in progress and the one before, so the dashboard has data whenever
-- the dev stack is started. Autumn runs from 1 September to 31 January,
-- spring from 1 February to 31 August, so every day belongs to a semester.
DO $$
DECLARE
    loc1_id INTEGER;
    loc2_id INTEGER;
    loc3_id INTEGER;
    room1_id INTEGER;
    room2_id INTEGER;
    sem_id INTEGER;
    concert1_id INTEGER;
    concert2_id INTEGER;
    concert3_date DATE;
    piece1_id INTEGER;
    piece2_id INTEGER;
    piece3_id INTEGER;
    ref_day DATE;
    person1_id INTEGER;
    person2_id INTEGER;
    person3_id INTEGER;
    sem_start DATE;
    sem_end DATE;
    sem_short_name TEXT;
    sem_long_name TEXT;
    anchor DATE;
    concert1_date DATE;
    concert2_date DATE;
BEGIN
    -- Locations
    IF NOT EXISTS (SELECT 1 FROM locations WHERE city = 'Paris' AND street = '1 Rue de la Paix') THEN
        INSERT INTO locations (name, city, zipcode, street, notes) VALUES
            ('Salle Gaveau', 'Paris', '75001', '1 Rue de la Paix', 'Entrée des artistes rue de Ponthieu. Piano de concert accordé sur demande.'),
            ('Opéra de Lyon', 'Lyon', '69001', '10 Place Bellecour', NULL),
            ('Conservatoire du 11e', 'Paris', '75011', '12 Rue Oberkampf', 'Clés à récupérer à l''accueil. Pas de parking.');

        SELECT id INTO loc1_id FROM locations WHERE city = 'Paris' AND street = '1 Rue de la Paix';
        SELECT id INTO loc2_id FROM locations WHERE city = 'Lyon'  AND street = '10 Place Bellecour';
        SELECT id INTO loc3_id FROM locations WHERE city = 'Paris' AND street = '12 Rue Oberkampf';

        -- Rooms without audience capacity are for rehearsals only.
        INSERT INTO rooms (location_id, name, musician_capacity, audience_capacity, total_capacity) VALUES
            (loc1_id, 'Grande salle', 80, 1000, 1050),
            (loc1_id, 'Salle Ravel', 40, NULL, NULL),
            (loc2_id, 'Grande salle', 90, 1100, NULL),
            (loc3_id, 'Auditorium', 70, 200, 250),
            (loc3_id, 'Salle 12', 60, NULL, NULL);

        INSERT INTO location_contacts (location_id, name, email, phone_number, role) VALUES
            (loc1_id, 'Claire Martin', 'claire.martin@example.com', '01 23 45 67 89', 'Régisseuse'),
            (loc1_id, 'Paul Durand', NULL, '06 12 34 56 78', NULL),
            (loc2_id, 'Sophie Bernard', 'sophie.bernard@example.com', NULL, 'Programmation');
    END IF;

    -- Concert rooms (absent if they were removed by hand).
    SELECT r.id INTO room1_id FROM rooms r JOIN locations l ON l.id = r.location_id
        WHERE l.street = '1 Rue de la Paix' AND r.name = 'Grande salle' AND r.audience_capacity IS NOT NULL;
    SELECT r.id INTO room2_id FROM rooms r JOIN locations l ON l.id = r.location_id
        WHERE l.street = '10 Place Bellecour' AND r.name = 'Grande salle' AND r.audience_capacity IS NOT NULL;

    -- Semesters: the previous one, then the current one.
    FOREACH ref_day IN ARRAY ARRAY[
        -- Last day of the previous semester.
        (CASE
            WHEN EXTRACT(MONTH FROM CURRENT_DATE) >= 9
                THEN make_date(EXTRACT(YEAR FROM CURRENT_DATE)::INT, 8, 31)
            WHEN EXTRACT(MONTH FROM CURRENT_DATE) = 1
                THEN make_date(EXTRACT(YEAR FROM CURRENT_DATE)::INT - 1, 8, 31)
            ELSE make_date(EXTRACT(YEAR FROM CURRENT_DATE)::INT, 1, 31)
        END),
        CURRENT_DATE
    ] LOOP
        IF EXTRACT(MONTH FROM ref_day) BETWEEN 2 AND 8 THEN
            sem_start := make_date(EXTRACT(YEAR FROM ref_day)::INT, 2, 1);
            sem_end := make_date(EXTRACT(YEAR FROM ref_day)::INT, 8, 31);
            sem_short_name := 'P' || to_char(sem_start, 'YY');
            sem_long_name := 'Printemps ' || to_char(sem_start, 'YYYY');
        ELSE
            sem_start := make_date(EXTRACT(YEAR FROM ref_day)::INT - (EXTRACT(MONTH FROM ref_day) = 1)::INT, 9, 1);
            sem_end := (sem_start + INTERVAL '5 months' - INTERVAL '1 day')::DATE;
            sem_short_name := 'A' || to_char(sem_start, 'YY');
            sem_long_name := 'Automne ' || to_char(sem_start, 'YYYY');
        END IF;

        -- Leave semesters created by hand alone (they cannot overlap).
        IF EXISTS (SELECT 1 FROM semesters WHERE start_date <= sem_end AND end_date >= sem_start) THEN
            CONTINUE;
        END IF;

        sem_id := NULL;
        INSERT INTO semesters (short_name, long_name, start_date, end_date)
            VALUES (sem_short_name, sem_long_name, sem_start, sem_end)
            ON CONFLICT DO NOTHING
            RETURNING id INTO sem_id;
        IF sem_id IS NULL THEN
            RAISE NOTICE 'Seed: semester % (or %) already exists, skipped', sem_short_name, sem_long_name;
            CONTINUE;
        END IF;

        -- Pieces
        INSERT INTO pieces (name, source, composer, semester_id)
            VALUES ('Symphony No. 5', 'Op. 67', 'Ludwig van Beethoven', sem_id)
            RETURNING id INTO piece1_id;
        INSERT INTO pieces (name, source, composer, semester_id)
            VALUES ('Bohemian Rhapsody', NULL, 'Freddie Mercury', sem_id)
            RETURNING id INTO piece2_id;
        INSERT INTO pieces (name, source, composer, semester_id)
            VALUES ('Carmen Suite', 'Bizet''s opera', 'Georges Bizet', sem_id)
            RETURNING id INTO piece3_id;

        IF room1_id IS NULL OR room2_id IS NULL THEN
            CONTINUE;
        END IF;

        -- Concerts around today: one a few weeks ago, one a few weeks ahead.
        -- In past semesters, both near the end.
        anchor := LEAST(GREATEST(CURRENT_DATE, sem_start), sem_end - 28);
        concert1_date := GREATEST(anchor - 21, sem_start);
        concert2_date := LEAST(anchor + 28, sem_end);

        -- The second room booking is not confirmed yet.
        INSERT INTO concerts (name, date, tickets_url, video_url, meeting_at, doors_open_at, starts_at, public, room_id, room_booked, semester_id)
            VALUES (
                CASE WHEN sem_short_name LIKE 'A%' THEN 'Concert d''automne' ELSE 'Concert de printemps' END,
                concert1_date, 'https://tickets.example.com/' || lower(sem_short_name), NULL, '18:00', '19:30', '20:00', TRUE, room1_id, TRUE, sem_id)
            RETURNING id INTO concert1_id;
        INSERT INTO concerts (date, tickets_url, video_url, meeting_at, doors_open_at, starts_at, public, room_id, room_booked, semester_id)
            VALUES (concert2_date, NULL, NULL, '16:30', NULL, '18:00', FALSE, room2_id, FALSE, sem_id)
            RETURNING id INTO concert2_id;

        INSERT INTO concert_pieces (concert_id, piece_id, position) VALUES
            (concert1_id, piece1_id, 0),
            (concert1_id, piece3_id, 1),
            (concert2_id, piece2_id, 0),
            (concert2_id, piece3_id, 1);

        -- In the current semester, a concert only planned so far: a date,
        -- without room nor start time.
        concert3_date := LEAST(anchor + 56, sem_end);
        IF ref_day = CURRENT_DATE AND concert3_date > concert2_date THEN
            INSERT INTO concerts (date, public, semester_id)
                VALUES (concert3_date, FALSE, sem_id);
        END IF;
    END LOOP;

    -- People credited on pieces, linked to the seeded pieces of every
    -- semester that have no credits yet.
    IF NOT EXISTS (SELECT 1 FROM people) THEN
        INSERT INTO people (first_name, last_name, anonymous, display_name) VALUES
            ('Jeanne', 'Moreau', FALSE, NULL),
            ('Lucas', 'Petit', FALSE, 'Luke Little'),
            ('Camille', 'Roux', TRUE, NULL);
    END IF;
    SELECT id INTO person1_id FROM people WHERE first_name = 'Jeanne' AND last_name = 'Moreau';
    SELECT id INTO person2_id FROM people WHERE first_name = 'Lucas' AND last_name = 'Petit';
    SELECT id INTO person3_id FROM people WHERE first_name = 'Camille' AND last_name = 'Roux';
    IF person1_id IS NOT NULL AND person2_id IS NOT NULL AND person3_id IS NOT NULL THEN
        INSERT INTO piece_people (piece_id, person_id, role)
        SELECT p.id, credit.person_id, credit.role::piece_role
        FROM pieces p
        JOIN (VALUES
            ('Symphony No. 5', person2_id, 'conductor'),
            -- Jeanne both arranges and conducts Bohemian Rhapsody.
            ('Bohemian Rhapsody', person1_id, 'arranger'),
            ('Bohemian Rhapsody', person1_id, 'conductor'),
            ('Carmen Suite', person3_id, 'arranger'),
            ('Carmen Suite', person1_id, 'arranger'),
            ('Carmen Suite', person2_id, 'conductor')
        ) AS credit (piece_name, person_id, role) ON credit.piece_name = p.name
        WHERE NOT EXISTS (SELECT 1 FROM piece_people pp WHERE pp.piece_id = p.id);
    END IF;
END $$;
