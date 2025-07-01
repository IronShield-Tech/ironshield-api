use axum::{Router, routing::{get, post}};

use crate::handler::{
    request::handle_ironshield_request,
    response::handle_response
};

pub fn app() -> Router {
    Router::new() 
//      .route("/request", post(handle_ironshield_request()))
        .route("/response", get(handle_response()))
}