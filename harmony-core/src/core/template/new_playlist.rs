use harmony_data::model::playlist::{NewPlaylistModel, PlaylistModel};

use crate::{
    core::{
        model::playlist::Playlist,
        validation::{result::ValidationError, validated_types::Name},
    },
    result::ServerError,
};

pub struct PlaylistTemplate {
    pub name: Name,
    pub owner: i32,
}

impl PlaylistTemplate {
    pub fn new(name: &str, owner: i32) -> Result<Self, ValidationError> {
        Ok(Self {
            name: Name::new(name)?,
            owner,
        })
    }

    pub fn create(&self) -> Result<Playlist, ServerError> {
        let new_playlist = self.to_model()?;

        match PlaylistModel::create(&new_playlist) {
            Ok(song) => Ok(Playlist::from_model(&song)),
            Err(_) => Err(ServerError::CouldNotCreate(
                "Failed to create playlist".to_string(),
            )),
        }
    }

    pub fn to_model(&self) -> Result<NewPlaylistModel, ServerError> {
        Ok(NewPlaylistModel {
            name: self.name.value().to_string(),
            owner: self.owner,
        })
    }
}
