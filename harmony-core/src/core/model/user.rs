use chrono::{DateTime, Utc};
use harmony_data::{model::user::UserModel, result::Error};
use serde::{Deserialize, Serialize};

use crate::result::ServerError;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub admin: bool,
    pub created: Option<DateTime<Utc>>,
}

impl User {
    pub fn get_list() -> Result<Vec<Self>, ServerError> {
        match UserModel::get_list() {
            Ok(users) => Ok(Self::from_models(&users)),
            Err(_) => Err(ServerError::Database("Failed to get info".to_string())),
        }
    }

    pub fn get_by_id(user_id: i32) -> Result<Self, ServerError> {
        let user = match UserModel::get_by_id(user_id) {
            Ok(user) => user,
            Err(error) => match error {
                Error::NotFound => return Err(ServerError::NotFound),
                _ => return Err(ServerError::Database("Failed to get user".to_string())),
            },
        };

        Ok(Self::from_model(&user))
    }

    pub fn find_by_name(name: &String) -> Result<Vec<Self>, ServerError> {
        let users = match UserModel::find(name) {
            Ok(users) => users,
            Err(error) => match error {
                Error::NotFound => return Err(ServerError::NotFound),
                _ => return Err(ServerError::Database("Failed to get user".to_string())),
            },
        };

        Ok(Self::from_models(&users))
    }

    pub fn from_model(user_model: &UserModel) -> Self {
        Self {
            id: user_model.id,
            name: user_model.name.clone(),
            admin: user_model.admin,
            created: user_model.created,
        }
    }

    pub fn from_models(user_models: &Vec<UserModel>) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();

        for model in user_models {
            result.push(Self::from_model(model));
        }

        result
    }
}
