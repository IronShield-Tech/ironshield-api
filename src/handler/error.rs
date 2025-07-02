use axum::{
    Json,
    http::StatusCode,
    response::{
        IntoResponse, 
        Response
    },
};
use thiserror::Error;


pub const MAX_TIME_DIFF_MS:  i64 = 3 * 10000; // 3 * 10,000 millis = 30 seconds
pub const PUB_KEY_FAIL:     &str = "Failed to load public key.";
pub const SIG_KEY_FAIL:     &str = "Failed to load signing key.";
pub const INVALID_ENDPOINT: &str = "Endpoint must be a valid HTTPS URL.";
pub const CLOCK_SKEW:       &str = "Request timestamp does not match the current time";

#[derive(Error, Debug)]
pub enum ErrorHandler {
    #[error("Invalid request format: {0}")]
    InvalidRequest(String),
    #[error("Processing failed: {0}")]
    ProcessingError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for ErrorHandler {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ErrorHandler::InvalidRequest(message) => {
                (StatusCode::BAD_REQUEST, message)
            },
            ErrorHandler::ProcessingError(message) => {
                (StatusCode::UNPROCESSABLE_ENTITY, message)
            },
            ErrorHandler::SerializationError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Data processing error".to_string())
            },
            ErrorHandler::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(serde_json::json!({
            "error": error_message,
            "success": false,
        }));

        (status, body).into_response()
    }
}