use harmony_core::core::template::new_song::SongTemplate;
use serde::{Deserialize, Serialize};

use crate::api_response::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSong {
    pub name: String,
    pub author: Option<String>,
    pub youtube_url: String,
}

impl NewSong {
    pub fn to_template(&self, added_by: i32) -> Result<SongTemplate, ApiResponse> {
        SongTemplate::new(
            &self.name,
            self.author.as_deref(),
            added_by,
            &self.youtube_url,
        )
        .map_err(ApiResponse::from)
    }
}
