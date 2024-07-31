#[macro_use]
extern crate rocket;
use harmony_api::error_response;
use harmony_api::handler::stream_handler::{self, stream_cleanup_task};
use harmony_api::handler::{playlist_handler, song_handler, user_handler, version_handler};
use harmony_api::utils::states::StreamState;

#[rocket::main]
async fn main() {
    let stream_state = StreamState::new();

    // Spawn the cleanup task
    let cleanup_handle = tokio::spawn(stream_cleanup_task(stream_state.clone()));

    let _rocket = rocket::build()
        .manage(stream_state)
        .register("/", catchers![error_response::catch_all])
        .mount("/", routes![version_handler::get_version_handler])
        .mount(
            "/song",
            routes![
                song_handler::list_song_handler,
                song_handler::get_song_handler,
                song_handler::find_song_handler,
                song_handler::create_song_handler,
            ],
        )
        .mount(
            "/user",
            routes![
                user_handler::list_users_handler,
                user_handler::login_user_handler,
                user_handler::get_user_handler,
                user_handler::create_user_handler,
                user_handler::find_user_handler
            ],
        )
        .mount(
            "/playlist",
            routes![
                playlist_handler::list_playlist_handler,
                playlist_handler::get_playlist_handler,
                playlist_handler::find_playlist_handler,
                playlist_handler::create_playlist_handler,
                playlist_handler::add_song_to_playlist_handler,
                playlist_handler::remove_song_from_playlist_handler
            ],
        )
        .mount(
            "/stream",
            routes![stream_handler::open_stream, stream_handler::read_stream],
        )
        .launch()
        .await;

    cleanup_handle.abort(); // Quit the thread
}
