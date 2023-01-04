-- Your SQL goes here
CREATE TYPE participation_type AS ENUM ('rejected', 'accepted', 'no_response');

CREATE TABLE groups_participants (
  id SERIAL PRIMARY KEY,
  group_id INT NOT NULL,
  participant_id INT NOT NULL,
  participation_type participation_type NOT NULL,
  FOREIGN KEY(group_id) REFERENCES groups(id),
  FOREIGN KEY(participant_id) REFERENCES users(id),

  CONSTRAINT unique_participants UNIQUE (group_id, participant_id)
)
