-- Your SQL goes here
ALTER TABLE bot
DROP COLUMN uuid;

ALTER TABLE bot
ADD COLUMN uuid UUID PRIMARY KEY DEFAULT gen_random_uuid();
