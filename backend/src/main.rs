mod app;
mod handler;
mod helper;
mod model;
mod route;

use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let app = app::create_app();
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let host = format!("{}:{}", host, port);
    tracing::info!("Starting server on {}", host);
    // Create a TCP listener on the specified host and port
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}