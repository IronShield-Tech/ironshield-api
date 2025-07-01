use thiserror::Error;


pub const MAX_TIME_DIFF_MS:  i64 = 3 * 10000; // 3 * 10,000 millis = 30 seconds
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