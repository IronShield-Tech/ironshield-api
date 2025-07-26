//! # Setup for Axum Routing.

use axum::{
    Router, 
    routing::post,
    routing::get
};
use tower_http::cors::{
    CorsLayer, 
    Any
};

use crate::constant::{
    HEALTH_ENDPOINT,
    REQUEST_ENDPOINT,
    RESPONSE_ENDPOINT
};
use crate::handler::{
    request::handle_challenge_request,
    response::handle_challenge_response,
    health::health_check
};

/// Creates a permissive CORS layer for development/testing purposes.
///
/// This configuration allows:
/// - Any origin to make requests.
/// - Any HTTP methods (GET, POST, etc.).
/// - Any headers in requests.
/// - Disables credentials whether that be cookies or authorization.
fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(false)
}

/// Creates and configures the main application router.
///
/// Binds (Prod):
/// * `/request`  to `handler::request::handle_challenge_request`.
/// * `/response` to `handler::response::handle_challenge_response`.
/// * `/health`   to `handler::health::health_check`.
///
/// Binds (Test):
/// * `/test/request`: to `test::endpoint::sample_request`.
pub fn app() -> Router {
    Router::new()
        .route(REQUEST_ENDPOINT,  post(handle_challenge_request))
        .route(RESPONSE_ENDPOINT, post(handle_challenge_response))
        .route(HEALTH_ENDPOINT,   get(health_check))
        
        .layer(create_cors_layer())
}
