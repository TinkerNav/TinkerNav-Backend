-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN uuid;

ALTER TABLE users
ADD COLUMN id SERIAL PRIMARY KEY;
