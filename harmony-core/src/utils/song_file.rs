use nanoid::nanoid;

use crate::result::ServerError;
use crate::tools::ffmpeg;
use crate::tools::file;
use crate::tools::ytdlp;

use super::environment;

fn download_song(youtube_url: &str) -> Result<String, ServerError> {
    let file_id = nanoid!();

    ytdlp::download_audio(
        youtube_url,
        &format!("{}/{}.mp3", environment::get_download_directory(), file_id).to_string(),
        &environment::get_max_download_filesize(),
    )?;

    Ok(file_id)
}

fn convert_song(song_id: &str) -> Result<(), ServerError> {
    ffmpeg::convert_to_dfpwm(
        &format!("{}/{}.mp3", environment::get_download_directory(), song_id).to_string(),
        &format!("{}/{}.dfpwm", environment::get_song_directory(), song_id).to_string(),
    )
}

fn clean_song(song_id: &str) -> Result<(), ServerError> {
    file::remove(&format!("{}/{}.mp3", environment::get_download_directory(), song_id).to_string())
}

pub fn get_song_data(file_id: &str) -> Result<Vec<u8>, ServerError> {
    file::read_data(&format!("{}/{}.dfpwm", environment::get_song_directory(), file_id).to_string())
}

/// Returns the file ID of the song pulled and converted
pub fn pull_song(youtube_url: &str) -> Result<String, ServerError> {
    let id = download_song(youtube_url)?;
    convert_song(&id)?;
    clean_song(&id)?;
    Ok(id)
}
