-- This file should undo anything in `up.sql`
ALTER TABLE posts 
    DROP COLUMN likes,
    ALTER COLUMN published SET DEFAULT TRUE
