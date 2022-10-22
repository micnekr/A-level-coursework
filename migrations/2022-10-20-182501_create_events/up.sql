-- Your SQL goes here
CREATE TYPE visibility_type AS ENUM ('public', 'private');
CREATE TYPE recurrence_type AS ENUM ('weekly', 'once');

CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  owner_id INT NOT NULL,
  title VARCHAR(100) NOT NULL,
  visibility visibility_type NOT NULL,
  start_time INT NOT NULL, -- a UNIX timestamp, in seconds
  duration INT NOT NULL, -- in seconds
  recurrence recurrence_type NOT NULL,
  FOREIGN KEY(owner_id) REFERENCES users(id)
)
