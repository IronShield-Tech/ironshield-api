use axum::Json;
use serde_json::{json, Value};

use ironshield_types::IronShieldRequest;

pub async fn sample_request() -> Json<Value> {
    let sample = IronShieldRequest::new(
        "https://exapmle.com/protected".to_string(),
        chrono::Utc::now().timestamp_millis(),
    );
    
    Json(json!({
        "description": "Sample request for IronShieldRequest structure.",
        "sample": sample,
        "usage": "POST this JSON structure to /request to get a challenge."
    }))
}