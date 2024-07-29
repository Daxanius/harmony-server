use harmony_core::core::model::user::User;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::api_response::ApiResponse;

use super::environment;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user: User, // User ID
    exp: usize, // Expiration time
}

pub fn get_jwt_token(user: &User) -> Result<String, ApiResponse> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + environment::get_token_time_valid(); // 1 week expiration

    let claims = Claims {
        user: user.clone(),
        exp: expiration as usize,
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(environment::get_jwt_secret().as_bytes()),
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err(ApiResponse::InternalServerError(
            "Could not create token".to_string(),
        )),
    }
}

pub fn get_user_from_jwt(token: &str) -> Result<User, ApiResponse> {
    let validation = Validation::default();
    let token_data: TokenData<Claims> = decode(
        token,
        &DecodingKey::from_secret(environment::get_jwt_secret().as_bytes()),
        &validation,
    )
    .map_err(|err| match *err.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            ApiResponse::Unauthorized("Token has expired".to_string())
        }
        _ => ApiResponse::Unauthorized("Invalid token".to_string()),
    })?;

    // Extract the user from the token's claims
    let user = token_data.claims.user;

    Ok(user)
}
