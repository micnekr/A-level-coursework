-- Your SQL goes here
CREATE TYPE participation_type AS ENUM ('invited', 'accepted', 'no_response');

CREATE TABLE groups_participants (
  id SERIAL PRIMARY KEY,
  group_id INT NOT NULL,
  participant_id INT NOT NULL,
  participation_type participation_type NOT NULL,
  FOREIGN KEY(group_id) REFERENCES groups(id),
  FOREIGN KEY(participant_id) REFERENCES users(id)
)
