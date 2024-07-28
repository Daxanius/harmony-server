use dotenvy::dotenv;
use std::env;

fn init_environment() {
    dotenv().ok();
}

/// # Panics
/// When the environment variable is not set
#[must_use]
pub fn get_max_download_filesize() -> String {
    init_environment();
    env::var("MAX_DOWNLOAD_FILESIZE").expect("MAX_DOWNLOAD_FILESIZE must be set.")
}

/// # Panics
/// When the environment variable is not set
#[must_use]
pub fn get_download_directory() -> String {
    init_environment();
    env::var("DOWNLOAD_DIRECTORY").expect("DOWNLOAD_DIRECTORY must be set.")
}

/// # Panics
/// When the environment variable is not set
#[must_use]
pub fn get_song_directory() -> String {
    init_environment();
    env::var("SONG_DIRECTORY").expect("SONG_DIRECTORY must be set.")
}
