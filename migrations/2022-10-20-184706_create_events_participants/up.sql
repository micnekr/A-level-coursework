-- Your SQL goes here
CREATE TYPE participation_type AS ENUM ('invited', 'accepted');

CREATE TABLE events_participants (
  id SERIAL PRIMARY KEY,
  event_id INT NOT NULL,
  participant_id INT NOT NULL,
  participation_type participation_type NOT NULL,
  FOREIGN KEY(event_id) REFERENCES events(id),
  FOREIGN KEY(participant_id) REFERENCES users(id)
)
