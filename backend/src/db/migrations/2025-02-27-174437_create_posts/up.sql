-- Your SQL goes here
CREATE TABLE posts (
  slug VARCHAR(255) PRIMARY KEY UNIQUE,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  author_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
  is_published BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP
);

CREATE TABLE tags (
  slug VARCHAR(255) PRIMARY KEY UNIQUE,
  name VARCHAR(255) NOT NULL,
  background_color CHAR(6) NOT NULL default '000000',
  foreground_color CHAR(6) NOT NULL default 'ffffff'
);

CREATE TABLE tags_to_posts (
  id SERIAL PRIMARY KEY,
  post VARCHAR(255) NOT NULL REFERENCES posts(slug) ON DELETE CASCADE,
  tag VARCHAR(255) NOT NULL REFERENCES tags(slug) ON DELETE CASCADE
);
