DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM semesters WHERE name = 'P26') THEN
        RETURN;
    END IF;

    INSERT INTO semesters (name, start_date, end_date)
    VALUES
    ('P26', '2026-01-28', '2026-07-04'),
    ('A25', '2025-09-01', '2026-01-28')
    ON CONFLICT (name) DO NOTHING;
END $$;

