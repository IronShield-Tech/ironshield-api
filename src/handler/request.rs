//! # Request handler and functions.

use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json;

use ironshield_types::{
    IronShieldChallenge, 
    IronShieldChallengeResponse, 
    IronShieldRequest
};
use crate::handler::error::{
    ErrorHandler, 
    CLOCK_SKEW, 
    INVALID_ENDPOINT, 
    MAX_TIME_DIFF_MS
};
use ironshield_cloudflare::{
    constant, 
    challenge
};
use crate::handler::result::ResultHandler;

use std::string::ToString;

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

pub fn handle_ironshield_request(
    Json(payload): Json<IronShieldRequest>,
) -> ResultHandler<Json<IronShieldChallengeResponse>> {
    // Validate the request.
    validate_ironshield_request(&payload)?;

    // This function will handle incoming requests.
    todo!("Validate the request then process the request and the Ok(Json whatever")
}

async fn process_ironshield_request(
    request: IronShieldRequest
) -> ResultHandler<IronShieldRequest> {
    todo!("Implement actual response creation based on IronShieldResponse type.")
}

fn validate_ironshield_request(
    request: &IronShieldRequest
) -> ResultHandler<()> {
    let time_diff = (chrono::Utc::now().timestamp_millis() - request.timestamp).abs();
    
    // Validate that request url comes from Hypertext Transfer Protocol Secure.
    if !request.endpoint.starts_with("https://") {
        return Err(ErrorHandler::InvalidRequest(
            INVALID_ENDPOINT.to_string(),
        ));
    }
    
    // Validate the request is not in the future or in the past.
    if time_diff < MAX_TIME_DIFF_MS {
        return Err(ErrorHandler::InvalidRequest(
            CLOCK_SKEW.to_string(),
        ))
    }
    
    Ok(())
}

async fn generate_challenge_for_request(
    request: IronShieldRequest
) -> ResultHandler<IronShieldChallenge> {
    let random_nonce = challenge::generate_random_nonce();
    let random_nonce = challenge::generate_random_nonce();
    let created_time = challenge::generate_created_time();
    let challenge_difficulty = constant::CHALLENGE_DIFFICULTY;
    let challenge_param = IronShieldChallenge::difficulty_to_challenge_param(challenge_difficulty);
    
    
    todo!("Implement actual response creation")
}