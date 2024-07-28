use chrono::{DateTime, Utc};
use harmony_data::{
    model::{
        playlist::PlaylistModel,
        playlist_song::{NewPlaylistSongModel, PlaylistSongModel},
    },
    result::Error,
};
use serde::{Deserialize, Serialize};

use crate::result::ServerError;

use super::song::Song;

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub owner: i32,
    pub public: bool,
    pub created: Option<DateTime<Utc>>,
}

impl Playlist {
    pub fn get_list(calling_user: i32) -> Result<Vec<Self>, ServerError> {
        match PlaylistModel::get_list(calling_user) {
            Ok(playlists) => Ok(Playlist::from_models(&playlists)),
            Err(_) => Err(ServerError::Database("Failed to get playlists".to_string())),
        }
    }

    pub fn get_by_id(playlist_id: i32) -> Result<Self, ServerError> {
        let user = match PlaylistModel::get_by_id(playlist_id) {
            Ok(user) => user,
            Err(error) => match error {
                Error::NotFound => return Err(ServerError::NotFound),
                _ => return Err(ServerError::Database("Failed to get user".to_string())),
            },
        };

        Ok(Self::from_model(&user))
    }

    pub fn find_by_name(
        playlist_name: &String,
        calling_user: i32,
    ) -> Result<Vec<Self>, ServerError> {
        let playlists = match PlaylistModel::find_by_name(playlist_name, calling_user) {
            Ok(playlists) => playlists,
            Err(error) => match error {
                Error::NotFound => return Err(ServerError::NotFound),
                _ => return Err(ServerError::Database("Failed to get user".to_string())),
            },
        };

        Ok(Self::from_models(&playlists))
    }

    pub fn find_by_user(user_id: i32, calling_user: i32) -> Result<Vec<Self>, ServerError> {
        match PlaylistModel::find_by_user(user_id, calling_user) {
            Ok(playlists) => Ok(Playlist::from_models(&playlists)),
            Err(_) => Err(ServerError::Database("Failed to get playlists".to_string())),
        }
    }

    pub fn remove(&self) -> Result<(), ServerError> {
        match PlaylistModel::remove(self.id) {
            Ok(()) => Ok(()),
            Err(_) => Err(ServerError::Database(
                "Failed to remove playlist".to_string(),
            )),
        }
    }

    pub fn get_songs(&self) -> Result<Vec<Song>, ServerError> {
        match PlaylistSongModel::get_songs_from_playlist(self.id) {
            Ok(playlists) => Ok(Song::from_models(&playlists)),
            Err(_) => Err(ServerError::Database("Failed to get songs".to_string())),
        }
    }

    /// Cecks if a song can be added to the playlist
    pub fn song_free(&self, song_id: i32) -> Result<(), ServerError> {
        match PlaylistSongModel::song_in_playlist(self.id, song_id) {
            Ok(()) => Err(ServerError::AlreadyExists),
            Err(err) => match err {
                Error::NotFound => Ok(()),
                _ => Err(ServerError::Database(
                    "Could not check if song is free".to_string(),
                )),
            },
        }
    }

    pub fn song_exists(&self, song_id: i32) -> Result<(), ServerError> {
        match PlaylistSongModel::song_in_playlist(self.id, song_id) {
            Ok(()) => Ok(()),
            Err(err) => match err {
                Error::NotFound => Err(ServerError::NotFound),
                _ => Err(ServerError::Database(
                    "Could not check if song is in playlist".to_string(),
                )),
            },
        }
    }

    pub fn add_song(&self, song_id: i32) -> Result<(), ServerError> {
        Song::get_by_id(song_id)?; // Check if the song exists
        self.song_free(song_id)?; // Check if the song isn't already in the playlist

        let sm = NewPlaylistSongModel {
            playlist: self.id,
            song: song_id,
        };

        match PlaylistSongModel::add_song_to_playlist(sm) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServerError::Database(
                "Failed to add song to playlist".to_string(),
            )),
        }
    }

    pub fn remove_song(&self, song_id: i32) -> Result<(), ServerError> {
        self.song_exists(song_id)?; // Check if the song is in the playlist

        match PlaylistSongModel::remove_song_from_playlist(self.id, song_id) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServerError::Database(
                "Failed to add song to playlist".to_string(),
            )),
        }
    }

    pub fn from_model(playlist_model: &PlaylistModel) -> Self {
        Self {
            id: playlist_model.id,
            name: playlist_model.name.clone(),
            owner: playlist_model.owner,
            public: playlist_model.public,
            created: playlist_model.created,
        }
    }

    pub fn from_models(playlist_models: &Vec<PlaylistModel>) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();

        for model in playlist_models {
            result.push(Self::from_model(model));
        }

        result
    }
}
