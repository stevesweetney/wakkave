-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    karma INTEGER NOT NULL DEFAULT 100,
    streak smallint NOT NULL DEFAULT 0
)