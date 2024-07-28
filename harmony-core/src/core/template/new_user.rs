use crate::core::model::user::User;
use crate::core::validation::result::ValidationError;
use crate::core::validation::validated_types::{Password, Username};
use crate::result::{AuthError, ServerError};
use bcrypt::{hash, verify, DEFAULT_COST};
use harmony_data::model::user::{NewUserModel, UserModel};
use harmony_data::result::Error;

pub struct UserTemplate {
    name: Username,
    password: Password,
}

impl UserTemplate {
    pub fn new(name: &str, password: &str) -> Result<Self, ValidationError> {
        Ok(Self {
            name: Username::new(name)?,
            password: Password::new(password)?,
        })
    }

    pub fn to_model(&self) -> Result<NewUserModel, ServerError> {
        Ok(NewUserModel {
            name: self.name.value().to_string(),
            hash: self.get_hash()?,
        })
    }

    pub fn get_hash(&self) -> Result<String, ServerError> {
        match hash(self.password.value(), DEFAULT_COST) {
            Ok(hash) => Ok(hash),
            Err(_) => Err(ServerError::HashFailed),
        }
    }

    /// Returns `ServerError::AlreadyExists` if the user exists
    pub fn name_free(&self) -> Result<(), ServerError> {
        match UserModel::exists(self.name.value()) {
            Ok(()) => Err(ServerError::AlreadyExists),
            Err(err) => match err {
                Error::NotFound => Ok(()),
                _ => Err(ServerError::Database(
                    "Failed to check if user exists".to_owned(),
                )),
            },
        }
    }

    pub fn exists(&self) -> Result<(), ServerError> {
        match UserModel::exists(self.name.value()) {
            Ok(()) => Ok(()),
            Err(err) => match err {
                Error::NotFound => Err(ServerError::NotFound),
                _ => Err(ServerError::Database(
                    "Failed to check if user exists".to_owned(),
                )),
            },
        }
    }

    /// Checks if the user provided valid login creddentials
    pub fn verify_auth(&self) -> Result<i32, AuthError> {
        if self.exists().is_err() {
            return Err(AuthError::UserNotFound);
        };

        let user_model: UserModel = match UserModel::get_by_name(self.name.value()) {
            Ok(user_model) => user_model,
            Err(_) => {
                return Err(AuthError::Database(
                    "Something went wrong during authentication".to_string(),
                ))
            }
        };

        // Verify the provided password against the stored hash
        if verify(self.password.value(), &user_model.hash).is_err() {
            return Err(AuthError::InvalidPassword);
        };

        Ok(user_model.id)
    }

    pub fn create(&self) -> Result<User, ServerError> {
        self.name_free()?; // The user cannot exist
        let new_user = self.to_model()?;

        match UserModel::create(new_user) {
            Ok(user) => Ok(User::from_model(&user)),
            Err(_) => Err(ServerError::CouldNotCreate(
                "Failed to create user".to_string(),
            )),
        }
    }
}
