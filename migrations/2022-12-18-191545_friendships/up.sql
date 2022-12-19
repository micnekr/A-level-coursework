-- Your SQL goes here
CREATE TABLE friendships (
  id SERIAL PRIMARY KEY,
  owner_id INT NOT NULL,
  friend_id INT NOT NULL,
  FOREIGN KEY(owner_id) REFERENCES users(id),
  FOREIGN KEY(friend_id) REFERENCES users(id)
)
