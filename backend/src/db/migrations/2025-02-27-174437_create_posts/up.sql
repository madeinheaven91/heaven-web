-- Your SQL goes here
CREATE TABLE posts (
  slug VARCHAR(255) PRIMARY KEY UNIQUE,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  author_id INTEGER NOT NULL REFERENCES users(id),
  is_published BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP
);

CREATE TABLE tags (
  slug VARCHAR(255) PRIMARY KEY UNIQUE,
  name VARCHAR(255) NOT NULL,
  background_color CHAR(6),
  foreground_color CHAR(6)
);

CREATE TABLE tags_to_posts (
  id SERIAL PRIMARY KEY,
  post VARCHAR(255) NOT NULL REFERENCES posts(slug),
  tag VARCHAR(255) NOT NULL REFERENCES tags(slug)
);
