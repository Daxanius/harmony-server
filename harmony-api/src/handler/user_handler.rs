use harmony_core::core::model::user::User;
use rocket::{get, post, serde::json::Json};

use crate::{
    api_response::ApiResponse, guards::token_auth::TokenAuth, model::new_user::NewUser,
    utils::auth_util,
};

#[get("/")]
pub fn list_users_handler(_auth: TokenAuth) -> Result<Json<Vec<User>>, ApiResponse> {
    let users: Vec<User> = User::get_list().map_err(ApiResponse::from)?;
    Ok(Json(users))
}

#[get("/id/<user_id>")]
pub fn get_user_handler(_auth: TokenAuth, user_id: i32) -> Result<Json<User>, ApiResponse> {
    match User::get_by_id(user_id) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e.into()),
    }
}

#[get("/name/<username>")]
pub fn find_user_handler(
    _auth: TokenAuth,
    username: String,
) -> Result<Json<Vec<User>>, ApiResponse> {
    let users: Vec<User> = User::find_by_name(&username).map_err(ApiResponse::from)?;
    Ok(Json(users))
}

#[post("/login", format = "application/json", data = "<user>")]
pub fn login_user_handler(user: Json<NewUser>) -> Result<String, ApiResponse> {
    let template = user.0.to_template()?;
    let user_id = template.verify_auth().map_err(ApiResponse::from)?;
    let user = User::get_by_id(user_id).map_err(ApiResponse::from)?;
    let token = auth_util::get_jwt_token(user)?;
    Ok(token)
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create_user_handler(user: Json<NewUser>) -> Result<Json<User>, ApiResponse> {
    let template = user.0.to_template()?;
    let user = template.create().map_err(ApiResponse::from)?;
    Ok(Json(user))
}
