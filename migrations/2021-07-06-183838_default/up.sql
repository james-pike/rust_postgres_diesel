-- Your SQL goes here
ALTER TABLE posts 
    ALTER COLUMN published SET DEFAULT FALSE,
    ADD COLUMN likes INTEGER NOT NULL DEFAULT 0
