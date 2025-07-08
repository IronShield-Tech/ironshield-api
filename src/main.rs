mod constant;
mod handler;
mod routing;
mod test;

use routing::app;

use tokio::net::TcpListener;
use tracing::{
    Level,
    info,
    error
};
use tracing_subscriber::FmtSubscriber;

use std::net::SocketAddr;

/// See https://docs.rs/axum/latest/axum/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)?;
    
    info!("Starting IronShield API Server v{}", env!("CARGO_PKG_VERSION"));

    // Build the router from the routing module.
    let app: axum::Router = app();

    // Define the address where the server will listen to.
    // We bind to the IPv6 "any" address `[::]`, which allows the Fly.io
    // proxy to connect to the app.
    let addr: SocketAddr = "[::]:3000".parse().expect("Failed to parse socket address");

    // Create a TCP listener bound to the address.
    let listener: TcpListener = TcpListener::bind(addr).await.unwrap_or_else(|e: std::io::Error| {
        error!("Failed to bind to address {}: {}", addr, e);
        panic!("Failed to bind to address {}: {}", addr, e);
    });
    
    info!("Axum API Web Server listening on http://{}", addr);
    
    axum::serve(listener, app).await.unwrap_or_else(|e: std::io::Error| {
        error!("server error: {}", e);
        panic!("Server error: {}", e);
    });
    
    Ok(())
}