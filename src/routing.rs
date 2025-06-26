use axum::{Router, routing::{get, post}};

use crate::handler::{
    request_handler::handle_request, 
    response_handler::handle_response
};

pub fn app() -> Router {
    Router::new()
        .route("/request", post(handle_request()))
        .route("/response", get(handle_response()))
}