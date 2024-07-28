use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{catch, Request};

#[derive(Serialize)]
pub struct ErrorResponse {
    status: u16,
    message: String,
}

#[catch(default)]
#[must_use]
pub fn catch_all(status: rocket::http::Status, _: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: status.code,
        message: format!("Error {}: {}", status.code, status.reason_lossy()),
    })
}
