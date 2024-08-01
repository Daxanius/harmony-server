use harmony_core::core::model::playlist::Playlist;
// use application::post::read;
use rocket::{delete, get, post, serde::json::Json};

use crate::{
    api_response::ApiResponse, guards::token_auth::TokenAuth, model::new_playlist::NewPlaylist,
};

#[get("/")]
pub fn list_playlist_handler(auth: TokenAuth) -> Result<Json<Vec<Playlist>>, ApiResponse> {
    let playlists: Vec<Playlist> = Playlist::get_list(auth.user.id).map_err(ApiResponse::from)?;
    Ok(Json(playlists))
}

#[get("/id/<playlist_id>")]
pub fn get_playlist_handler(
    auth: TokenAuth,
    playlist_id: i32,
) -> Result<Json<Playlist>, ApiResponse> {
    match Playlist::get_by_id(playlist_id) {
        Ok(playlist) => {
            if playlist.public || (playlist.owner == auth.user.id) {
                return Ok(Json(playlist));
            }

            Err(ApiResponse::Unauthorized("Playlist is private".to_string()))
        }
        Err(e) => Err(e.into()),
    }
}

#[post("/id/<playlist_id>/<song_id>")]
pub fn add_song_to_playlist_handler(
    auth: TokenAuth,
    playlist_id: i32,
    song_id: i32,
) -> Result<(), ApiResponse> {
    let playlist = Playlist::get_by_id(playlist_id)?;
    if playlist.owner != auth.user.id {
        return Err(ApiResponse::Unauthorized(
            "Not the owner of this playlist".to_string(),
        ));
    };

    playlist.add_song(song_id)?;
    Ok(())
}

#[delete("/id/<playlist_id>/<song_id>")]
pub fn remove_song_from_playlist_handler(
    auth: TokenAuth,
    playlist_id: i32,
    song_id: i32,
) -> Result<(), ApiResponse> {
    let playlist = Playlist::get_by_id(playlist_id)?;
    if playlist.owner != auth.user.id {
        return Err(ApiResponse::Unauthorized(
            "Not the owner of this playlist".to_string(),
        ));
    };

    playlist.remove_song(song_id)?;
    Ok(())
}

#[get("/name/<name>")]
pub fn find_playlist_handler(
    auth: TokenAuth,
    name: &str,
) -> Result<Json<Vec<Playlist>>, ApiResponse> {
    let playlists: Vec<Playlist> =
        Playlist::find_by_name(&name, auth.user.id).map_err(ApiResponse::from)?;
    Ok(Json(playlists))
}

#[get("/user/<user_id>")]
pub fn find_playlist_by_user_handler(
    auth: TokenAuth,
    user_id: i32,
) -> Result<Json<Vec<Playlist>>, ApiResponse> {
    let playlists: Vec<Playlist> =
        Playlist::find_by_user(user_id, auth.user.id).map_err(ApiResponse::from)?;
    Ok(Json(playlists))
}

#[post("/", format = "application/json", data = "<playlist>")]
pub fn create_playlist_handler(
    auth: TokenAuth,
    playlist: Json<NewPlaylist>,
) -> Result<Json<Playlist>, ApiResponse> {
    let template = playlist.0.to_template(auth.user.id)?;
    let playlist = template.create().map_err(ApiResponse::from)?;
    Ok(Json(playlist))
}

#[delete("/id/<playlist_id>")]
pub fn remove_playlist_handler(auth: TokenAuth, playlist_id: i32) -> Result<(), ApiResponse> {
    let playlist = Playlist::get_by_id(playlist_id)?;
    if playlist.owner != auth.user.id {
        return Err(ApiResponse::Unauthorized(
            "Not the owner of this playlist".to_string(),
        ));
    }

    playlist.remove()?;
    Ok(())
}
