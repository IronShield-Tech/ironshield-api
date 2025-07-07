use axum::{
    Router, 
    routing::post,
    routing::get
};
use tower_http::cors::{
    CorsLayer, 
    Any
};

use crate::handler::{
    request::handle_challenge_request,
    response::handle_challenge_response,
    health::health_check
};
use crate::test;

fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(false)
}

pub fn app() -> Router {
    Router::new()
        .route("/request", post(handle_challenge_request))
        .route("/response", post(handle_challenge_response))
        .route("/health", get(health_check))
        .route("/test/request", get(test::endpoint::sample_request))
        
        .layer(create_cors_layer())
}