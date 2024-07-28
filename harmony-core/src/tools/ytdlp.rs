use crate::result::ServerError;
use std::process::Command;

// Download a video and return the filename
pub fn download_audio(
    youtube_url: &str,
    output_file: &str,
    max_filesize: &str,
) -> Result<(), ServerError> {
    let status = Command::new("yt-dlp")
        .args([
            "-x",
            "--audio-format",
            "mp3",
            "--max-filesize",
            max_filesize,
            "-o",
            output_file,
            youtube_url,
        ])
        .status()
        .expect("Failed to execute yt-dlp command");

    if status.success() {
        return Ok(());
    }

    Err(ServerError::ExecutionFailed)
}
