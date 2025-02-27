-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(32) NOT NULL,
  password VARCHAR(255) NOT NULL,
  email VARCHAR(64),
  is_staff BOOLEAN NOT NULL
);
