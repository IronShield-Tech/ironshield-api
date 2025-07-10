//! # Request handler and functions.

use axum::extract::Json;
use serde_json::{
    json,
    Value
};

use ironshield_types::{
    load_private_key_from_env,
    load_public_key_from_env,
    IronShieldChallenge,
    IronShieldRequest
};
use crate::constant;
use crate::handler::{
    error::{
        ErrorHandler, 
        CLOCK_SKEW, 
        INVALID_ENDPOINT, 
        MAX_TIME_DIFF_MS,
    },
    result::ResultHandler
};

use std::string::ToString;

pub async fn handle_challenge_request(
    Json(payload): Json<IronShieldRequest>,
) -> ResultHandler<Json<Value>> {
    // Validate the request.
    validate_challenge_request(&payload)?;
    
    // Process the request and generate a challenge.
    let challenge: IronShieldChallenge = generate_challenge_for_request(payload).await?;

    // Return the challenge response.
    Ok(Json(json!({
        "status": constant::STATUS_OK,
        "message": constant::STATUS_OK_MSG,
        "challenge": challenge
    })))
}

fn validate_challenge_request(
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
    if time_diff > MAX_TIME_DIFF_MS {
        return Err(ErrorHandler::InvalidRequest(
            CLOCK_SKEW.to_string(),
        ))
    }
    
    Ok(())
}

async fn generate_challenge_for_request(
    request: IronShieldRequest
) -> ResultHandler<IronShieldChallenge> {
    // Load the signing key from the env var.
    let signing_key = load_private_key_from_env()
        .map_err(|e| ErrorHandler::ProcessingError(format!("Failed to load signing key: {}", e)))?;
    
    // Load the public key from the env var.
    let public_key = load_public_key_from_env()
        .map_err(|e| ErrorHandler::ProcessingError(format!("Failed to load public key: {}", e)))?;

    let challenge_param = IronShieldChallenge::difficulty_to_challenge_param(ironshield_types::CHALLENGE_DIFFICULTY);
    
    // Create the challenge using the construction from ironshield-types.
    // This constructor automatically:
    // - Generates a random nonce using IronShieldChallenge::generate_random_nonce().
    // - Sets created_time using IronShieldChallenge::generate_created_time().
    // - Sets expiration_time to created_time + 30 seconds.
    // - Signs the challenge with the provided signing key.
    let mut challenge = IronShieldChallenge::new(
        request.endpoint.clone(),
        challenge_param,
        signing_key,
        public_key.to_bytes(),
    );
    
    // Set the challenge properties based on the difficulty.
    challenge.set_recommended_attempts(ironshield_types::CHALLENGE_DIFFICULTY);
    
    // TODO: Store challenge in a cache for later verification.
    
    Ok(challenge)
}