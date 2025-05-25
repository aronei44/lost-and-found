mod app;
mod handler;
mod helper;
mod model;
mod route;
mod data;
mod api_doc;

use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;
use std::net::SocketAddr;

// Import the Parser derive macro and trait from clap
use clap::Parser;
use std::path::Path;


use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!();

/// Server CLI
#[derive(Parser, Debug)]
struct Args {
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    #[clap(long, default_value = "8001")]
    port: u16,
}



#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    
    let migration_flag = Path::new(".migrated");

    if std::env::var("MIGRATION") == Ok("true".to_string()) && !migration_flag.exists() {
        // Run migrations
        let pool = helper::db::create_pool().await.unwrap_or_else(|err| {
            eprintln!("Failed to create database pool: {}", err);
            std::process::exit(1);
        });
        // Run migrations
        MIGRATOR.run(&pool).await.unwrap_or_else(|err| {
            eprintln!("Failed to run migrations: {}", err);
        });

        std::fs::write(migration_flag, "done").unwrap();
    }


    let app = app::create_app();
    // Default values
    let args = Args::parse();

    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse().expect("Invalid host or port");

    tracing::info!("Starting server on {}", addr);
    // Create a TCP listener on the specified host and port
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}