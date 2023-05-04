-- Your SQL goes here
-- ""Visibility type" refers to who gets the permission to view this event
CREATE TYPE visibility_type AS ENUM ('public', 'private');
-- "Recurrence type" refers to how often this event is repeated.
-- For example, a "once" event is not repeated at all
CREATE TYPE recurrence_type AS ENUM ('weekly', 'once');

-- A table used to store events
CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  visibility visibility_type NOT NULL,
  start_time INT NOT NULL, -- a UNIX timestamp, in seconds
  duration INT NOT NULL, -- in seconds
  recurrence recurrence_type NOT NULL,
  group_id INT NOT NULL,
  FOREIGN KEY(group_id) REFERENCES groups(id)
)
