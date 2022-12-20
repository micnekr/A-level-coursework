-- Your SQL goes here
CREATE TABLE groups (
  id SERIAL PRIMARY KEY,
  is_special BOOLEAN NOT NULL, -- e.g. "myself" group, managed by the program and not the user
  name VARCHAR(100) NOT NULL,
  owner_id INT NOT NULL,
  FOREIGN KEY(owner_id) REFERENCES users(id)
)
