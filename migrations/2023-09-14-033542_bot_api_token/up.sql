-- Your SQL goes here
CREATE TABLE bot_api_token (
    uuid TEXT NOT NULL,
    bot_uuid TEXT NOT NULL,
    token TEXT NOT NULL,
    PRIMARY KEY (uuid)
);
