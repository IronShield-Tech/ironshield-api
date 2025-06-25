use axum::routing::get;

use std::fs::OpenOptions;
use std::net::SocketAddr;
use std::io::Write;
use tokio::net::TcpListener;

/// See https://docs.rs/axum/latest/axum/
#[tokio::main]
async fn main() {
    // Set up a log file for debugging purposes.
    let mut log_file: std::fs::File = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("debug_log.txt")
        .expect("Failed to open log file.");

    // Log the server start time.
    writeln!(&log_file, "Server starting at {:?}", std::time::SystemTime::now())
        .expect("Failed to write to log file.");

    // Build the router from the routing module.
    let app: axum::Router = app();

    // Define the address where the server will listen to.
    // In this case, it listens on all interfaces
    // at port 3000.
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Create a TCP listener bound to the address.
    let listener: TcpListener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            writeln!(&mut log_file, "Failed to bind to address: {}: {}", addr, e)
                .expect("Failed to write to log file.");
            panic!("Failed to bind to address: {}", e);
        });
    
    writeln!(&mut log_file, "Backend listening on http://{}", addr)
        .expect("Failed to write to log file.");
    
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| {
            writeln!(&mut log_file, "Server error: {}", e)
                .expect("Failed to write to log file.");
            panic!("Server error: {}", e);
        });
}

pub fn app() -> axum::Router {
    axum::Router::new()
        .route("/request", get(ironshield_api::handler::request_handler::handle_request()))
        .route("/response", get(ironshield_api::handler::response_handler::handle_response()))
}