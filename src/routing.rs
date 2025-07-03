use axum::{Router, routing::post};

use crate::handler::{
    request::handle_ironshield_request,
    response::handle_response
};

pub fn app() -> Router {
    Router::new()
        .route("/request", post(handle_ironshield_request))
        .route("/response", post(handle_response()))
}