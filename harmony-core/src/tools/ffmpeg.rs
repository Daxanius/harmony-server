use crate::result::ServerError;
use std::process::Command;

// Download a video and return the filename
pub fn convert_to_dfpwm(file: &str, output_file: &str) -> Result<(), ServerError> {
    let status = Command::new("ffmpeg")
        .args([
            "-i",
            file,
            "-ac",
            "1",
            "-ar",
            "48k",
            "-c:a",
            "dfpwm",
            output_file,
        ])
        .status()
        .expect("Failed to execute ffmpeg command");

    if status.success() {
        return Ok(());
    }

    Err(ServerError::ExecutionFailed)
}
