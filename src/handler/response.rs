//! # Response Handler and functions.

use axum::extract::Json;

#[allow(unused_imports)]
use ironshield_core::verify_ironshield_solution;
use ironshield_types::{
    load_private_key_from_env,
    load_public_key_from_env,
    IronShieldChallengeResponse,
    IronShieldToken
};
use ironshield::handler::{
    error::{
        ErrorHandler,
        CHALLENGE_EXPIRED,
        PUB_KEY_FAIL,
        SIG_KEY_FAIL,
        INVALID_SOLUTION,
        INVALID_ENDPOINT,
        INVALID_PARAMS,
        SIGNATURE_FAIL,
        STATUS_OK,
        STATUS_OK_MSG,
        CHALLENGE_EXPIRED_MSG,
        INVALID_SOLUTION_MSG,
        INVALID_ENDPOINT_MSG,
        INVALID_PARAMS_MSG,
    },
    result::ResultHandler
};

use serde_json::{
    json,
    Value
};

// Response types for OpenAPI documentation that use actual error constants
#[derive(utoipa::IntoResponses)]
#[allow(dead_code)]
enum ResponseResponses {
    /// Solution verified and token generated
    #[response(status = 200)]
    Success {
        status: u16,
        message: String,
        token: ironshield_types::IronShieldToken,
    },
    /// Invalid solution provided for the challenge
    #[response(status = 422, description = INVALID_SOLUTION_MSG)]
    InvalidSolution,
    /// Challenge has expired
    #[response(status = 410, description = CHALLENGE_EXPIRED_MSG)]
    ChallengeExpired,
    /// Endpoint must be a valid HTTPS URL
    #[response(status = 422, description = INVALID_ENDPOINT_MSG)]
    InvalidEndpoint,
    /// Invalid challenge parameters
    #[response(status = 400, description = INVALID_PARAMS_MSG)]
    InvalidParams,
}

#[utoipa::path(
    post,
    path = "/response",
    request_body = IronShieldChallengeResponse,
    responses(ResponseResponses),
    tag = "Challenge"
)]
pub async fn handle_challenge_response(
    Json(payload): Json<IronShieldChallengeResponse>,
) -> ResultHandler<Json<Value>> {
    // Validate the challenge response structure and content.
    validate_challenge_response(&payload)?;

    // TODO: Verify the proof-of-work solution.
    // verify_ironshield_solution(&payload);

    // Verify the proof-of-work solution and generate a token.
    let token: IronShieldToken = generate_authentication_token(payload).await?;

    // Return the authentication token.
    Ok(Json(json!({
        "status":  STATUS_OK,
        "message": STATUS_OK_MSG,
        "token":   token
    })))
}

fn validate_challenge_response(
    response: &IronShieldChallengeResponse
) -> ResultHandler<()> {
    if response.solution < 0 {
        return Err(ErrorHandler::InvalidRequest(format!("{}: {}", INVALID_SOLUTION.message, response.solution)))
    }
    if response.solved_challenge.is_expired() {
        return Err(ErrorHandler::InvalidRequest(CHALLENGE_EXPIRED.to_string()))
    }
    if response.solved_challenge.website_id.is_empty() {
        return Err(ErrorHandler::InvalidRequest(INVALID_ENDPOINT.to_string()))
    }
    if response.solved_challenge.challenge_param == [0u8; 32] {
        return Err(ErrorHandler::InvalidRequest(INVALID_PARAMS.to_string()));
    }

    Ok(())
}

async fn generate_authentication_token(
    response: IronShieldChallengeResponse
) -> ResultHandler<IronShieldToken> {
    // TODO: Retrieve the original challenge from the cache.
    // TODO: Verify the solution using ironshield-core.

    // Allow for one-hour validity for the token.
    let valid_for: i64 = chrono::Utc::now().timestamp_millis() + (60 * 60 * 1000);

    // Signatures should cover challenge_signature + valid_for
    // to prevent tampering.
    let signing_key: ironshield_core::SigningKey = load_private_key_from_env()
        .map_err(|e: ironshield_types::CryptoError| ErrorHandler::ProcessingError(format!("{}: {}", SIG_KEY_FAIL.message, e)))?;
    let public_key: [u8; 32] = load_public_key_from_env()
        .map_err(|e: ironshield_types::CryptoError| ErrorHandler::ProcessingError(format!("{}: {}", PUB_KEY_FAIL.message, e)))?
        .to_bytes();

    let auth_msg: String = format!(
        "{}|{}",
        hex::encode(response.solved_challenge.challenge_signature),
        valid_for
    );

    // Signing the authentication message.
    let auth_signature: [u8; 64] = ironshield_types::generate_signature(&signing_key, &auth_msg)
        .map_err(|e: ironshield_types::CryptoError| ErrorHandler::ProcessingError(format!("{}: {}", SIGNATURE_FAIL.message, e)))?;

    let token: IronShieldToken = IronShieldToken::new(
        response.solved_challenge.challenge_signature,
        valid_for,
        public_key,
        auth_signature,
    );

    Ok(token)
}