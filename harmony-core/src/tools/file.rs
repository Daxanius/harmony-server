use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::result::ServerError;

// Cleans up the audio from the downloads folder
pub fn remove(file: &str) -> Result<(), ServerError> {
    // Check if the file exists and then remove it
    if Path::new(&file).exists() {
        fs::remove_file(file).map_err(|_| ServerError::ExecutionFailed)?;
        Ok(())
    } else {
        Err(ServerError::FileNotFound)
    }
}

// Read a specified number of chunks from the converted audio file
pub fn read_data(file: &str) -> Result<Vec<u8>, ServerError> {
    // Check if the file exists
    if !Path::new(&file).exists() {
        return Err(ServerError::FileNotFound);
    }

    // Open the file
    let mut file = File::open(file).map_err(|_| ServerError::ExecutionFailed)?;

    // Read the entire file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|_| ServerError::ExecutionFailed)?;

    Ok(buffer)
}
