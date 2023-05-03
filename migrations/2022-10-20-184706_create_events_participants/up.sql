-- Your SQL goes here
-- A linking table used to store user-event pairs to denote what users participate in what events
-- Note that the owner of the event is automatically considered a participant, no entry in events_participants required.
CREATE TABLE events_participants (
  id SERIAL PRIMARY KEY,
  event_id INT NOT NULL, -- A foreign key to the event
  participant_id INT NOT NULL, -- A foreign key to the participant user
  FOREIGN KEY(event_id) REFERENCES events(id),
  FOREIGN KEY(participant_id) REFERENCES users(id)
)
