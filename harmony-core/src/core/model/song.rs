use chrono::{DateTime, Utc};
use harmony_data::{model::song::SongModel, result::Error};
use serde::{Deserialize, Serialize};

use crate::result::ServerError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Song {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub added_by: i32,
    pub youtube_url: String,
    pub file_id: String,
    pub created: Option<DateTime<Utc>>,
}

impl Song {
    pub fn get_list() -> Result<Vec<Self>, ServerError> {
        let Ok(songs) = SongModel::get_list() else {
            return Err(ServerError::Database("Failed to get songs".to_string()));
        };

        Ok(Self::from_models(&songs))
    }

    pub fn find(query: &str) -> Result<Vec<Self>, ServerError> {
        let Ok(songs) = SongModel::find(query) else {
            return Err(ServerError::Database("Failed to get songs".to_string()));
        };

        Ok(Self::from_models(&songs))
    }

    pub fn get_by_id(song_id: i32) -> Result<Self, ServerError> {
        let song = match SongModel::get_by_id(song_id) {
            Ok(song) => song,
            Err(error) => match error {
                Error::NotFound => return Err(ServerError::NotFound),
                _ => return Err(ServerError::Database("Failed to get song".to_string())),
            },
        };

        Ok(Self::from_model(&song))
    }

    pub fn from_model(song_model: &SongModel) -> Self {
        Self {
            id: song_model.id,
            name: song_model.name.clone(),
            author: song_model.author.clone(),
            added_by: song_model.added_by,
            youtube_url: song_model.youtube_url.clone(),
            file_id: song_model.file_id.clone(),
            created: song_model.created,
        }
    }

    pub fn from_models(playlist_models: &Vec<SongModel>) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();

        for model in playlist_models {
            result.push(Self::from_model(model));
        }

        result
    }
}
