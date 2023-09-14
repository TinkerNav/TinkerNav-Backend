-- Your SQL goes here
-- Create bot table
CREATE TABLE IF NOT EXISTS bot (
    uuid TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY (uuid)
);
