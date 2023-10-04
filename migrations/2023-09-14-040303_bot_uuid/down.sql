-- This file should undo anything in `up.sql`
ALTER TABLE bot
DROP COLUMN uuid;

ALTER TABLE bot
ADD COLUMN uuid TEXT PRIMARY KEY;

