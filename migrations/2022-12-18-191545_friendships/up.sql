-- Your SQL goes here
-- A linking table that stores a friendship link between users
CREATE TABLE friendships (
  id SERIAL PRIMARY KEY,
  -- Both are foreign keys to the user table
  owner_id INT NOT NULL,
  friend_id INT NOT NULL,
  FOREIGN KEY(owner_id) REFERENCES users(id),
  FOREIGN KEY(friend_id) REFERENCES users(id)
)
