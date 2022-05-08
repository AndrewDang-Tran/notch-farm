-- Add migration script here
CREATE TABLE IF NOT EXISTS arguments (
    argument_id INTEGER PRIMARY KEY,
    group_id INTEGER NOT NULL UNIQUE,
    argument_starter INTEGER NOT NULL,
    dissenter INTEGER NOT NULL,
    description TEXT NOT NULL,
    status TEXT CHECK(status IN ('InProgress', 'NotchTaken')) NOT NULL,
    notch_taker INTEGER
)