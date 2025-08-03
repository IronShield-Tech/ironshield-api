//! # Health check handler module.

use axum::Json;
use serde_json::{
    json, 
    Value
};

use crate::constant;
use ironshield::handler::{
    result::ResultHandler,
    error::STATUS_OK
};

// Response types for OpenAPI documentation
#[derive(utoipa::IntoResponses)]
#[allow(dead_code)]
enum HealthResponses {
    /// Health check successful
    #[response(status = 200)]
    Success {
        status: u16,
        service: String,
        version: String,
        timestamp: i64,
    },
}

/// Health check endpoint.
/// 
/// # Returns:
/// * `Json<Value>`: A JSON object containing the 
///                  - health status, 
///                  - service name, 
///                  - version,
///                  - current timestamp.
#[utoipa::path(
    get,
    path = "/health",
    responses(HealthResponses),
    tag = "Health"
)]
pub async fn health_check() -> ResultHandler<Json<Value>> {
    Ok(Json(json!({
        "status":    STATUS_OK,
        "service":   constant::SERVICE_NAME,
        "version":   constant::VERSION,
        "timestamp": chrono::Utc::now().timestamp_millis()
    })))
}
