//! # Setup for Axum Routing.

use axum::{
    Router, 
    routing::post,
    routing::get,
    response::Response,
    http::{StatusCode, header}
};
use tower_http::cors::{
    CorsLayer, 
    Any
};
use utoipa::OpenApi;
use serde_json;

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
        (name = "Health", description = "Health check endpoint"),
        (name = "Challenge", description = "Challenge generation and verification endpoints")
    ),
    info(
        title = "IronShield API",
        version = env!("CARGO_PKG_VERSION"),
        description = "A stateless scraping & L7 DDoS protection solution optimized for performance, privacy, and accessibility"
    )
)]
struct ApiDoc;

/// Serves the favicon.ico file (embedded at compile time)
async fn favicon() -> Response<axum::body::Body> {
    // Embed the favicon at compile time to ensure it's available in production
    const FAVICON_DATA: &[u8] = include_bytes!("../assets/favicon.ico");
    
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/x-icon")
        .header(header::CACHE_CONTROL, "public, max-age=86400") // Cache for 1 day
        .body(axum::body::Body::from(FAVICON_DATA))
        .unwrap()
}

/// Serves the OpenAPI JSON specification
async fn openapi_json() -> Response<axum::body::Body> {
    let openapi = ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&openapi).unwrap();
    
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "public, max-age=300") // Cache for 5 minutes
        .body(axum::body::Body::from(json))
        .unwrap()
}

/// Serves a custom Swagger UI HTML page with IronShield branding
async fn custom_swagger_ui() -> Response<axum::body::Body> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="description" content="IronShield RESTAPI Swagger UI" />
    <title>REST API | IronShield</title>
    <link rel="icon" type="image/x-icon" href="/favicon.ico">
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
    <style>
        .swagger-ui .info .title {
            color: #0074E9;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        }
        .swagger-ui .info .description {
            color: #333;
        }
        .swagger-ui .scheme-container {
            background: linear-gradient(90deg, #0074E9 0%, #9ECDFC 100%);
            border: none;
            box-shadow: 0 2px 4px rgba(0,116,233,0.1);
        }
        .swagger-ui .topbar {
            background: linear-gradient(90deg, #0074E9 0%, #9ECDFC 100%);
            border-bottom: 1px solid #e3e3e3;
        }
        .swagger-ui .topbar .download-url-wrapper .download-url-button {
            background: #0074E9;
            border-color: #0074E9;
        }
        .swagger-ui .btn.authorize {
            background: #0074E9;
            border-color: #0074E9;
        }
        .swagger-ui .btn.execute {
            background: #0074E9;
            border-color: #0074E9;
        }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js" crossorigin></script>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-standalone-preset.js" crossorigin></script>
    <script>
        window.onload = () => {
            window.ui = SwaggerUIBundle({
                url: '/api-docs/openapi.json',
                dom_id: '#swagger-ui',
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                layout: "StandaloneLayout",
                deepLinking: true,
                showExtensions: true,
                showCommonExtensions: true,
                tryItOutEnabled: true,
                filter: true,
                requestInterceptor: (request) => {
                    // Add any custom request headers here if needed
                    return request;
                },
                responseInterceptor: (response) => {
                    // Add any custom response handling here if needed
                    return response;
                }
            });
        };
    </script>
</body>
</html>"#;

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .header(header::CACHE_CONTROL, "public, max-age=300") // Cache for 5 minutes
        .body(axum::body::Body::from(html))
        .unwrap()
}

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
        .route("/favicon.ico",    get(favicon))
        
        // Add custom Swagger UI route and OpenAPI route
        .route("/", get(custom_swagger_ui))
        .route("/api-docs/openapi.json", get(openapi_json))
        
        .layer(create_cors_layer())
}
