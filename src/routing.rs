use axum::{
    Router, 
    routing::post
};

use crate::handler::{
    request::handle_challenge_request,
    response::handle_challenge_response
};

pub fn app() -> Router {
    Router::new()
        .route("/request", post(handle_challenge_request))
        .route("/response", post(handle_challenge_response))
}