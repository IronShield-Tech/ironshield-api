use ironshield_api::routing::app;

use tokio::net::TcpListener;
use tracing::{
    Level,
    info,
    error
};
use tracing_subscriber::FmtSubscriber;

use std::{
    fs::OpenOptions,
    net::SocketAddr,
    io::Write
};

/// See https://docs.rs/axum/latest/axum/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)?;
    
    // Set up a log file for debugging purposes.
    let mut log_file: std::fs::File = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("debug_log.txt")
        .expect("Failed to open log file.");

    // Log the server start time.
    writeln!(
        &log_file, 
        "Server starting at {:?}",
        std::time::SystemTime::now()
    ).expect("Failed to write to log file.");
    
    info!("Starting IronShield API Server v{}", env!("CARGO_PKG_VERSION"));

    // Build the router from the routing module.
    let app = app();

    // Define the address where the server will listen to.
    // In this case, it listens on all interfaces
    // at port 3000.
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Create a TCP listener bound to the address.
    let listener: TcpListener = TcpListener::bind(addr).await.unwrap_or_else(|e| {
        error!("Failed to bind to address {}: {}", addr, e);
        writeln!(&mut log_file, "Failed to bind to address {}: {}", addr, e)
            .expect("Failed to write to log file.");
        panic!("Failed to bind to address {}: {}", addr, e);
    });
    
    info!("API server listening on http://{}", addr);
    
    writeln!(&mut log_file, "Backend listening on http://{}", addr)
        .expect("Failed to write to log file.");

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        error!("server error: {}", e);
        writeln!(&mut log_file, "Server error: {}", e).expect("Failed to write to log file.");
        panic!("Server error: {}", e);
    });
    
    Ok(())
}