-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  dob VARCHAR NOT NULL,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL
)