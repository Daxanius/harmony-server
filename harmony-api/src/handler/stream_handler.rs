use harmony_core::utils::song_file;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket::{get, post};
use std::time::{Duration, Instant};

use crate::api_response::ApiResponse;
use crate::utils::environment;
use crate::utils::states::StreamState;

#[post("/open/<file_id>")]
pub async fn open_stream(
    file_id: &str,
    state: &rocket::State<StreamState>,
) -> Result<Json<usize>, ApiResponse> {
    let mut file_data_map = state.file_data.write().await;
    let id_str = file_id.to_string();

    // Check if the file stream already exists
    if let Some((data, expiration_time)) = file_data_map.get_mut(&id_str) {
        // Update the expiration time
        *expiration_time = Instant::now() + Duration::from_secs(environment::get_stream_lifetime());
        Ok(Json(data.len()))
    } else {
        // File stream does not exist, read the file data
        let file_data: Vec<u8> = song_file::get_song_data(file_id).map_err(ApiResponse::from)?;
        let size = file_data.len();

        let expiration_time =
            Instant::now() + Duration::from_secs(environment::get_stream_lifetime()); // 5 minutes TTL
        file_data_map.insert(id_str, (file_data, expiration_time));

        Ok(Json(size))
    }
}

#[get("/read/<file_id>?<start>&<length>")]
pub async fn read_stream(
    file_id: &str,
    start: Option<usize>,
    length: Option<usize>,
    state: &rocket::State<StreamState>,
) -> Result<(ContentType, Vec<u8>), ApiResponse> {
    let mut file_data_map = state.file_data.write().await;
    let Some((file_data, expiration_time)) = file_data_map.get_mut(&file_id.to_string()) else {
        return Err(ApiResponse::NotFound(
            "Could not find open stream".to_string(),
        ));
    };

    // Reset expiration time
    *expiration_time = Instant::now() + Duration::from_secs(environment::get_stream_lifetime());

    let start = start.unwrap_or(0);

    if start >= file_data.len() {
        return Err(ApiResponse::BadRequest("Range not satisfiable".to_string()));
    }

    let end = match length {
        Some(length) => start.saturating_add(length).min(file_data.len()),
        None => file_data.len(),
    };

    let chunk = file_data[start..end].to_vec();

    Ok((ContentType::Binary, chunk))
}
