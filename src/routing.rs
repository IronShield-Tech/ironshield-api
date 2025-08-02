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
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handler::health::health_check,
        crate::handler::request::handle_challenge_request,
        crate::handler::response::handle_challenge_response,
    ),
    components(
        schemas(
            ironshield_types::IronShieldRequest,
            ironshield_types::IronShieldChallenge,
            ironshield_types::IronShieldChallengeResponse,
            ironshield_types::IronShieldToken,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "challenges", description = "Challenge generation and verification endpoints")
    ),
    info(
        title = "IronShield API",
        version = "0.2.4",
        description = "A stateless scraping & L7 DDoS protection solution optimized for performance, privacy, and accessibility",
        license(name = "SSPL v1.0", url = "https://github.com/IronShield-Tech/ironshield-api/blob/main/LICENSE")
    )
)]
struct ApiDoc;

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
/// * `/` to Swagger UI interface (root path).
/// * `/api-docs/openapi.json` to OpenAPI specification.
pub fn app() -> Router {
    Router::new()
        .route(REQUEST_ENDPOINT,  post(handle_challenge_request))
        .route(RESPONSE_ENDPOINT, post(handle_challenge_response))
        .route(HEALTH_ENDPOINT,   get(health_check))
        
        // Add Swagger UI at root path
        .merge(SwaggerUi::new("/")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        
        .layer(create_cors_layer())
}
