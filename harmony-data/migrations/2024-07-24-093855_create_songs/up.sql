-- Your SQL goes here

CREATE TABLE songs (
  id SERIAL PRIMARY KEY,
  name VARCHAR(32) NOT NULL,
  author VARCHAR(255),
  added_by SERIAL not null,
  youtube_url VARCHAR(255) not null unique,
  file_id VARCHAR(60) not null unique,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_song_user_id FOREIGN KEY (added_by) REFERENCES users(id)
)