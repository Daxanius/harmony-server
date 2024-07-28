use harmony_core::core::validation::result::ValidationError;
use harmony_core::result::{AuthError, ServerError};
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use std::io::Cursor;

#[derive(Debug)]
pub enum ApiResponse {
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
    Conflict(String),
    Unauthorized(String),
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, _request: &Request<'_>) -> Result<Response<'static>, Status> {
        match self {
            ApiResponse::BadRequest(message) => Response::build()
                .status(Status::BadRequest)
                .sized_body(message.len(), Cursor::new(message))
                .ok(),
            ApiResponse::NotFound(message) => Response::build()
                .status(Status::NotFound)
                .sized_body(message.len(), Cursor::new(message))
                .ok(),
            ApiResponse::InternalServerError(message) => Response::build()
                .status(Status::InternalServerError)
                .sized_body(message.len(), Cursor::new(message))
                .ok(),
            ApiResponse::Conflict(message) => Response::build()
                .status(Status::Conflict)
                .sized_body(message.len(), Cursor::new(message))
                .ok(),
            ApiResponse::Unauthorized(message) => Response::build()
                .status(Status::Unauthorized)
                .sized_body(message.len(), Cursor::new(message))
                .ok(),
        }
    }
}

impl From<ServerError> for ApiResponse {
    fn from(error: ServerError) -> Self {
        match error {
            ServerError::Database(msg) => ApiResponse::InternalServerError(msg),
            ServerError::CouldNotCreate(msg) => ApiResponse::BadRequest(msg),
            ServerError::HashFailed => {
                ApiResponse::InternalServerError("Hash operation failed".into())
            }
            ServerError::AlreadyExists => ApiResponse::Conflict("Resource already exists".into()),
            ServerError::NotFound => ApiResponse::NotFound("Resource not found".into()),
            ServerError::ExecutionFailed => {
                ApiResponse::InternalServerError("Execution failed".into())
            }
            ServerError::FileNotFound => ApiResponse::NotFound("File not found".into()),
            _ => ApiResponse::InternalServerError("Failed".into()),
        }
    }
}

impl From<AuthError> for ApiResponse {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::InvalidPassword => ApiResponse::BadRequest("Invalid password".into()),
            AuthError::UserNotFound => ApiResponse::NotFound("User not found".into()),
            AuthError::HashFailed => {
                ApiResponse::InternalServerError("Hash operation failed".into())
            }
            AuthError::Database(msg) => ApiResponse::InternalServerError(msg),
            _ => ApiResponse::InternalServerError("Failed".into()),
        }
    }
}

impl From<ValidationError> for ApiResponse {
    fn from(error: ValidationError) -> Self {
        match error {
            ValidationError::InvalidUserName(msg) => {
                ApiResponse::BadRequest(format!("Invalid username: {msg}"))
            }
            ValidationError::InvalidPassword(msg) => {
                ApiResponse::BadRequest(format!("Invalid password: {msg}"))
            }
            ValidationError::InvalidEmail(msg) => {
                ApiResponse::BadRequest(format!("Invalid email: {msg}"))
            }
            ValidationError::InvalidFilePath(msg) => {
                ApiResponse::BadRequest(format!("Invalid file path: {msg}"))
            }
            ValidationError::InvalidYouTubeUrl(msg) => {
                ApiResponse::BadRequest(format!("Invalid YouTube URL: {msg}"))
            }
            ValidationError::InvalidName(msg) => {
                ApiResponse::BadRequest(format!("Invalid name: {msg}"))
            }
            _ => ApiResponse::InternalServerError("Failed".into()),
        }
    }
}
