-- This file should undo anything in `up.sql`
ALTER TABLE users 
ALTER COLUMN password_hash 
TYPE VARCHAR(255);
