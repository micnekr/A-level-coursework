-- Your SQL goes here
CREATE TABLE groups (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  owner_id INT NOT NULL,
  FOREIGN KEY(owner_id) REFERENCES users(id)
)
