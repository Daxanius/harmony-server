use harmony_core::core::template::new_user::UserTemplate;
use serde::{Deserialize, Serialize};

use crate::api_response::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    name: String,
    password: String,
}

impl NewUser {
    pub fn to_template(&self) -> Result<UserTemplate, ApiResponse> {
        UserTemplate::new(&self.name, &self.password).map_err(ApiResponse::from)
    }
}
