-- Your SQL goes here
-- What is the status of the user in the group? Have they accepted the invitation?
CREATE TYPE participation_type AS ENUM ('rejected', 'accepted', 'no_response');

-- A linking table that links groups and users that participate in them
CREATE TABLE groups_participants (
  id SERIAL PRIMARY KEY,
  group_id INT NOT NULL,
  participant_id INT NOT NULL,
  -- Also need participation type on top of that
  participation_type participation_type NOT NULL,
  FOREIGN KEY(group_id) REFERENCES groups(id),
  FOREIGN KEY(participant_id) REFERENCES users(id),

  CONSTRAINT unique_participants UNIQUE (group_id, participant_id)
)
