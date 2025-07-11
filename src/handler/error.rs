//! # Error Handling enum and constants.

use axum::{
    Json,
    http::StatusCode,
    response::{
        IntoResponse, 
        Response
    },
};
use thiserror::Error;

use std::time::Duration;

pub const  MAX_TIME_DIFF_MS:  i64 = 3 * 10000; // 3 * 10,000 milliseconds = 30 seconds
pub const      PUB_KEY_FAIL: &str = "Failed to load public key";
pub const      SIG_KEY_FAIL: &str = "Failed to load signing key";
pub const    SIGNATURE_FAIL: &str = "Signature verification failed";
pub const  INVALID_ENDPOINT: &str = "Endpoint must be a valid HTTPS URL";
pub const        CLOCK_SKEW: &str = "Request timestamp does not match the current time";
pub const  INVALID_SOLUTION: &str = "Invalid solution provided for the challenge";

// Extended error types for projects that reference this API.

pub const     NETWORK_ERROR: &str = "Network request failed";
pub const     TIMEOUT_ERROR: &str = "Operation timed out";
pub const      CONFIG_ERROR: &str = "Invalid configuration";
pub const CHALLENGE_EXPIRED: &str = "Challenge has expired";
pub const    MAX_ITERATIONS: &str = "Maximum solving iterations reached without finding solution";

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

    // Extended error types for projects that reference this API.

    #[error("Network request failed: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Operation timed out after {duration:?}")]
    TimeoutError { duration: Duration },
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Challenge solving failed: {0}")]
    ChallengeSolvingError(String),
    #[error("Challenge verification failed: {0}")]
    ChallengeVerificationError(String),
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),
    #[error("Resource not found: {0}")]
    NotFoundError(String),
    #[error("Permission denied: {0}")]
    PermissionError(String),
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

            // Extended error type checking for projects that reference this API.

            ErrorHandler::NetworkError(_) => {
                (StatusCode::BAD_GATEWAY, "Network communication failed".to_string())
            }
            ErrorHandler::TimeoutError { duration: _ } => {
                (StatusCode::REQUEST_TIMEOUT, "Request timed out".to_string())
            }
            ErrorHandler::ConfigurationError(message) => {
                (StatusCode::BAD_REQUEST, format!("Configuration error: {}", message))
            }
            ErrorHandler::ChallengeSolvingError(message) => {
                (StatusCode::UNPROCESSABLE_ENTITY, format!("Challenge solving failed: {}", message))
            },
            ErrorHandler::ChallengeVerificationError(message) => {
                (StatusCode::UNAUTHORIZED, format!("Challenge verification failed: {}", message))
            },
            ErrorHandler::AuthenticationError(message) => {
                (StatusCode::UNAUTHORIZED, format!("Authentication failed: {}", message))
            },
            ErrorHandler::RateLimitError(message) => {
                (StatusCode::TOO_MANY_REQUESTS, format!("Rate limit exceeded: {}", message))
            },
            ErrorHandler::NotFoundError(message) => {
                (StatusCode::NOT_FOUND, format!("Resource not found: {}", message))
            },
            ErrorHandler::PermissionError(message) => {
                (StatusCode::FORBIDDEN, format!("Permission denied: {}", message))
            },
        };

        let body = Json(serde_json::json!({
            "error": error_message,
            "success": false,
        }));

        (status, body).into_response()
    }
}

impl ErrorHandler {
    /// # Arguments
    /// * `error`: A `reqwest` network error.
    ///
    /// # Returns
    /// * `Self`: An `ErrorHandler::NetworkError` passed with the
    ///           argument provided to this function.
    pub fn from_network_error(error: reqwest::Error) -> Self {
        ErrorHandler::NetworkError(error)
    }

    /// # Arguments
    /// * `message`: The duration of the request.
    ///
    /// # Returns
    /// * `Self`: An `ErrorHandler::TimeoutError` passed with the
    ///           argument provided to this function.
    pub fn timeout(duration: Duration) -> Self {
        ErrorHandler::TimeoutError { duration }
    }

    /// # Arguments
    /// * `message`: The error message thrown on the event
    ///              configuration fails.
    ///
    /// # Returns
    /// * `Self`: An `ErrorHandler::ConfigurationError` passed
    ///           with the argument provided to this function.
    pub fn config_error(message: impl Into<String>) -> Self {
        ErrorHandler::ConfigurationError(message.into())
    }

    /// # Arguments
    /// * `message`: The error message thrown on the event
    ///              solving a challenge fails.
    ///
    /// # Returns
    /// * `Self`: An `ErrorHandler::ChallengeSolvingError` passed with
    ///           argument provided to this function.
    pub fn challenge_solving_error(message: impl Into<String>) -> Self {
        ErrorHandler::ChallengeSolvingError(message.into())
    }

    /// # Arguments
    /// * `message`: The error message thrown on the event
    ///              verification of a challenge fails.
    ///
    /// # Returns
    /// * `Self`: An `ErrorHandler::ChallengeVerificationError` passed
    ///           with the argument provided to this function.
    pub fn challenge_verification_error(message: impl Into<String>) -> Self {
        ErrorHandler::ChallengeVerificationError(message.into())
    }

    /// # Arguments
    /// * `message`: The error message thrown on the event
    ///              authentication fails.
    ///
    /// # Returns
    /// * `Self`: An `ErrorHandler::AuthenticationError` passed with
    ///           the argument provided to this function.
    pub fn authentication_error(message: impl Into<String>) -> Self {
        ErrorHandler::AuthenticationError(message.into())
    }

    /// # Arguments
    /// * `message`: The error message thrown on the event
    ///              a rate limit error occurs.
    ///
    /// # Returns
    /// * `Self`: An `ErrorHandler::RateLimitError` passed with
    ///           the argument provided to this function.
    pub fn rate_limit_error(message: impl Into<String>) -> Self {
        ErrorHandler::RateLimitError(message.into())
    }

    /// # Arguments
    /// * `message`: The error message thrown on the event
    ///              a `404` or "not found" error occurs.
    ///
    /// # Returns
    /// * `Self`: An `ErrorHandler::NotFoundError` passed with
    ///           the argument provided to this function.
    pub fn not_found_error(message: impl Into<String>) -> Self {
        ErrorHandler::NotFoundError(message.into())
    }

    /// # Arguments
    /// * `message`: The error message thrown on the event
    ///              that a permission error occurs.
    ///
    /// # Returns
    /// * `Self`: An `ErrorHandler::PermissionError` passed with
    ///           the argument provided to this function.
    pub fn permission_error(message: impl Into<String>) -> Self {
        ErrorHandler::PermissionError(message.into())
    }
}