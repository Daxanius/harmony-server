use crate::api_response::ApiResponse;
use crate::utils::auth_util;
use harmony_core::core::model::user::User;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAuth {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TokenAuth {
    type Error = ApiResponse;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");

        match token {
            Some(token) => {
                let token = token.trim_start_matches("Bearer ");

                match auth_util::get_user_from_jwt(token) {
                    Ok(user) => Outcome::Success(TokenAuth { user }),
                    Err(_) => Outcome::Error((
                        Status::Unauthorized,
                        ApiResponse::Unauthorized("Invalid token".to_string()),
                    )),
                }
            }
            None => Outcome::Error((
                Status::Unauthorized,
                ApiResponse::Unauthorized("Missing token".to_string()),
            )),
        }
    }
}
