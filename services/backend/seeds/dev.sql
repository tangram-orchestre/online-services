DO $$
DECLARE
    loc1_id INTEGER;
    loc2_id INTEGER;
    concert1_id INTEGER;
    concert2_id INTEGER;
    piece1_id INTEGER;
    piece2_id INTEGER;
    piece3_id INTEGER;
BEGIN
    -- Semesters
    IF NOT EXISTS (SELECT 1 FROM semesters WHERE name = 'P26') THEN
        INSERT INTO semesters (name, start_date, end_date) VALUES
            ('P26', '2026-01-28', '2026-07-04'),
            ('A25', '2025-09-01', '2026-01-28')
        ON CONFLICT (name) DO NOTHING;
    END IF;

    -- Locations
    IF NOT EXISTS (SELECT 1 FROM locations WHERE city = 'Paris' AND street = '1 Rue de la Paix') THEN
        INSERT INTO locations (city, zipcode, street) VALUES
            ('Paris', '75001', '1 Rue de la Paix'),
            ('Lyon', '69001', '10 Place Bellecour');

        SELECT id INTO loc1_id FROM locations WHERE city = 'Paris' AND street = '1 Rue de la Paix';
        SELECT id INTO loc2_id FROM locations WHERE city = 'Lyon'  AND street = '10 Place Bellecour';

        -- Concerts
        INSERT INTO concerts (date, tickets_url, video_url, doors_open_at, starts_at, public, location_id) VALUES
            ('2026-05-15', 'https://tickets.example.com/spring-concert', NULL, '19:30', '20:00', TRUE,  loc1_id),
            ('2026-06-20', NULL,                                          NULL, NULL,   '18:00', FALSE, loc2_id);

        SELECT id INTO concert1_id FROM concerts WHERE date = '2026-05-15' AND location_id = loc1_id;
        SELECT id INTO concert2_id FROM concerts WHERE date = '2026-06-20' AND location_id = loc2_id;

        -- Pieces
        INSERT INTO pieces (name, source, composer, arranger) VALUES
            ('Symphony No. 5',    'Op. 67',         'Ludwig van Beethoven', NULL),
            ('Bohemian Rhapsody', NULL,              'Freddie Mercury',      'John Smith'),
            ('Carmen Suite',      'Bizet''s opera',  'Georges Bizet',        'Pierre Dupont');

        SELECT id INTO piece1_id FROM pieces WHERE name = 'Symphony No. 5';
        SELECT id INTO piece2_id FROM pieces WHERE name = 'Bohemian Rhapsody';
        SELECT id INTO piece3_id FROM pieces WHERE name = 'Carmen Suite';

        INSERT INTO concert_pieces (concert_id, piece_id) VALUES
            (concert1_id, piece1_id),
            (concert1_id, piece3_id),
            (concert2_id, piece2_id),
            (concert2_id, piece3_id);
    END IF;
END $$;
