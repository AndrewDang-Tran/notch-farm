-- Add migration script here
CREATE TABLE IF NOT EXISTS arguments (
     argument_id INTEGER PRIMARY KEY,
     guild_id INTEGER NOT NULL,
     argument_starter_id INTEGER NOT NULL,
     dissenter_id INTEGER NOT NULL,
     description TEXT NOT NULL,
     status TEXT CHECK(status IN ('InProgress', 'NotchTaken')) NOT NULL,
     notch_taker_id INTEGER
)