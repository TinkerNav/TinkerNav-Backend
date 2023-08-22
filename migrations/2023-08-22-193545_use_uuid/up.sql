-- Your SQL goes here
ALTER TABLE users
DROP COLUMN id;

ALTER TABLE users
ADD COLUMN uuid UUID PRIMARY KEY DEFAULT gen_random_uuid();
