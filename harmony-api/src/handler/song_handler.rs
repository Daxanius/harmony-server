use harmony_core::core::model::song::Song;
// use application::post::read;
use rocket::{get, post, serde::json::Json};
use tokio::task;

use crate::{api_response::ApiResponse, guards::token_auth::TokenAuth, model::new_song::NewSong};

#[get("/")]
pub fn list_song_handler(_auth: TokenAuth) -> Result<Json<Vec<Song>>, ApiResponse> {
    let songs: Vec<Song> = Song::get_list().map_err(ApiResponse::from)?;
    Ok(Json(songs))
}

#[get("/id/<song_id>")]
pub fn get_song_handler(_auth: TokenAuth, song_id: i32) -> Result<Json<Song>, ApiResponse> {
    match Song::get_by_id(song_id) {
        Ok(song) => Ok(Json(song)),
        Err(e) => Err(e.into()),
    }
}

#[get("/find/<query>")]
pub fn find_song_handler(_auth: TokenAuth, query: &str) -> Result<Json<Vec<Song>>, ApiResponse> {
    let songs: Vec<Song> = Song::find(&query).map_err(ApiResponse::from)?;
    Ok(Json(songs))
}

#[post("/", format = "application/json", data = "<song>")]
pub async fn create_song_handler(
    auth: TokenAuth,
    song: Json<NewSong>,
) -> Result<Json<Song>, ApiResponse> {
    let template = song.0.to_template(auth.user.id)?;

    // Spawn a blocking task for the create operation
    let blocking_task = task::spawn_blocking(move || template.create());

    // Await the result of the blocking task
    let song = blocking_task.await.map_err(ApiResponse::from)??;

    Ok(Json(song))
}
