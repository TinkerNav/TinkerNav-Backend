-- This file should undo anything in `up.sql`
ALTER TABLE bot_api_token
DROP COLUMN bot_uuid;

ALTER TABLE bot_api_token
ADD COLUMN bot_uuid TEXT;

ALTER TABLE bot_api_token
DROP COLUMN uuid;

ALTER TABLE bot_api_token
ADD COLUMN uuid TEXT PRIMARY KEY;
