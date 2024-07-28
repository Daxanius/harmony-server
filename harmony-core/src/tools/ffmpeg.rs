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
            "-c:a",
            "dfpwm",
            output_file,
            "-ar",
            "48k",
        ])
        .status()
        .expect("Failed to execute ffmpeg command");

    if status.success() {
        return Ok(());
    }

    Err(ServerError::ExecutionFailed)
}
