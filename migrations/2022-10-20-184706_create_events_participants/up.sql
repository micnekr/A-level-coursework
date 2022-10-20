-- Your SQL goes here
CREATE TABLE events_participants (
  id SERIAL PRIMARY KEY,
  event_id INT NOT NULL,
  participant_id INT NOT NULL,
  FOREIGN KEY(event_id) REFERENCES events(id),
  FOREIGN KEY(participant_id) REFERENCES users(id)
)
