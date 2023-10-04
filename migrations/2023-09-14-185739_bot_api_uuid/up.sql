-- Your SQL goes here
ALTER TABLE bot_api_token
DROP COLUMN bot_uuid;

ALTER TABLE bot_api_token
ADD COLUMN bot_uuid UUID NOT NULL REFERENCES bot(uuid);

ALTER TABLE bot_api_token
DROP COLUMN uuid;

ALTER TABLE bot_api_token
ADD COLUMN uuid UUID PRIMARY KEY DEFAULT gen_random_uuid();
