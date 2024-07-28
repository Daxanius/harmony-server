-- Your SQL goes here

CREATE TABLE playlists (
  id SERIAL PRIMARY KEY,
  name VARCHAR(32) NOT NULL,
  owner SERIAL NOT NULL,
  public BOOL NOT NULL DEFAULT FALSE,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_playlist_user_id FOREIGN KEY (owner) REFERENCES users(id)
)