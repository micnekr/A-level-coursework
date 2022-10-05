-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(50) NOT NULL UNIQUE,
  password_hash TEXT NOT NULL
  -- body TEXT NOT NULL,
  -- published BOOLEAN NOT NULL DEFAULT FALSE
)
