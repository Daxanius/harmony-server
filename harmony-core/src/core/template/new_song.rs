use harmony_data::model::song::{NewSongModel, SongModel};
use harmony_data::result::Error;

use crate::core::model::song::Song;
use crate::core::validation::result::ValidationError;
use crate::core::validation::validated_types::{Name, YouTubeUrl};
use crate::result::ServerError;
use crate::utils;

pub struct SongTemplate {
    pub name: Name,
    pub author: Option<Name>,
    pub added_by: i32,
    pub youtube_url: YouTubeUrl,
}

impl SongTemplate {
    pub fn new(
        name: &str,
        author: Option<&str>,
        user: i32,
        url: &str,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            name: Name::new(name)?,
            author: match author {
                None => None,
                Some(val) => Some(Name::new(val)?),
            },
            added_by: user,
            youtube_url: YouTubeUrl::new(url)?,
        })
    }

    /// Returns `ServerError::AlreadyExists` if the song exists
    pub fn song_free(&self) -> Result<(), ServerError> {
        match SongModel::exists(self.youtube_url.value()) {
            Ok(()) => Err(ServerError::AlreadyExists),
            Err(err) => match err {
                Error::NotFound => Ok(()),
                _ => Err(ServerError::Database(
                    "Failed to check if song exists".to_owned(),
                )),
            },
        }
    }

    pub fn create(&self) -> Result<Song, ServerError> {
        self.song_free()?;
        let new_song = self.to_model()?;

        match SongModel::create(&new_song) {
            Ok(song) => Ok(Song::from_model(&song)),
            Err(_) => Err(ServerError::CouldNotCreate(
                "Failed to create song".to_string(),
            )),
        }
    }

    pub fn to_model(&self) -> Result<NewSongModel, ServerError> {
        Ok(NewSongModel {
            name: self.name.value().to_string(),
            author: self.author.as_ref().map(|a| a.value().clone()),
            added_by: self.added_by,
            youtube_url: self.youtube_url.value().clone(),
            file_id: utils::song_file::pull_song(self.youtube_url.value())?,
        })
    }
}
