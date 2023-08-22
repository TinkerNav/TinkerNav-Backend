-- Your SQL goes here
ALTER TABLE users 
ALTER COLUMN password_hash 
TYPE VARCHAR(512);
