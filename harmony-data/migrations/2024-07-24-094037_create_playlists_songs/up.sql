-- Your SQL goes here
CREATE TABLE playlists_songs (
  id SERIAL PRIMARY KEY,
  playlist SERIAL NOT NULL,
  song SERIAL NOT NULL,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_playlist_song_playlist_id FOREIGN KEY (playlist) REFERENCES playlists(id),
  CONSTRAINT fk_playlist_song_song_id FOREIGN KEY (song) REFERENCES songs(id)
)