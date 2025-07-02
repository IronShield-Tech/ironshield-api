//! # Request handler and functions.

use axum::extract::Json;

use ironshield_types::{
    load_private_key_from_env, 
    load_public_key_from_env, 
    IronShieldChallenge, 
    IronShieldChallengeResponse, 
    IronShieldRequest
};
use crate::handler::{
    error::{
        ErrorHandler, 
        CLOCK_SKEW, 
        INVALID_ENDPOINT, 
        MAX_TIME_DIFF_MS, 
        PUB_KEY_FAIL, 
        SIG_KEY_FAIL
    },
    result::ResultHandler
};

use std::string::ToString;

pub async fn handle_ironshield_request(
    Json(payload): Json<IronShieldRequest>,
) -> ResultHandler<Json<IronShieldChallengeResponse>> {
    // Validate the request.
    validate_ironshield_request(&payload)?;
    
    let challenge = generate_challenge_for_request(payload).await?;

    todo!("impl")
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
    // Load the signing key from the env var.
    let signing_key = load_private_key_from_env()
        .map_err(|e| ErrorHandler::ProcessingError(format!("{}: {}", SIG_KEY_FAIL, e)))?;
    
    // Load the public key from the env var.
    let public_key = load_public_key_from_env()
        .map_err(|e| ErrorHandler::ProcessingError(format!("{}: {}", PUB_KEY_FAIL, e)))?;

    let challenge_param = IronShieldChallenge::difficulty_to_challenge_param(ironshield_types::CHALLENGE_DIFFICULTY);
    
    let mut challenge = IronShieldChallenge::new(
        request.endpoint.clone(),
        challenge_param,
        signing_key,
        public_key.to_bytes(),
    );
    
    // Set the challenge properties based on the difficulty.
    challenge.set_recommended_attempts(ironshield_types::CHALLENGE_DIFFICULTY);
    
    Ok(challenge)
}