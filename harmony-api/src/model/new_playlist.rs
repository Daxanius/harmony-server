use harmony_core::core::template::new_playlist::PlaylistTemplate;
use serde::{Deserialize, Serialize};

use crate::api_response::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPlaylist {
    pub name: String,
}

impl NewPlaylist {
    pub fn to_template(&self, owner: i32) -> Result<PlaylistTemplate, ApiResponse> {
        PlaylistTemplate::new(&self.name, owner).map_err(ApiResponse::from)
    }
}
