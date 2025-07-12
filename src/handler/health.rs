//! # Health check handler module.

use axum::Json;
use serde_json::{
    json, 
    Value
};

use crate::constant;
use crate::handler::result::ResultHandler;

/// Health check endpoint.
/// 
/// # Returns:
/// * `Json<Value>`: A JSON object containing the 
///                  - health status, 
///                  - service name, 
///                  - version,
///                  - current timestamp.
pub async fn health_check() -> ResultHandler<Json<Value>> {
    Ok(Json(json!({
        "status":    constant::STATUS_OK,
        "service":   constant::SERVICE_NAME,
        "version":   constant::VERSION,
        "timestamp": chrono::Utc::now().timestamp_millis()
    })))
}
