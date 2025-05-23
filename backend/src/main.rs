mod app;
mod handler;
mod helper;
mod model;
mod route;
mod data;

use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;
use std::net::SocketAddr;

// Import the Parser derive macro and trait from clap
use clap::Parser;

/// Server CLI
#[derive(Parser, Debug)]
struct Args {
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    #[clap(long, default_value = "8000")]
    port: u16,
}



#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let app = app::create_app();
    // Default values
    let args = Args::parse();

    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse().expect("Invalid host or port");

    tracing::info!("Starting server on {}", addr);
    // Create a TCP listener on the specified host and port
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}