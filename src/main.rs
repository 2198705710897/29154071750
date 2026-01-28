#[cfg(feature = "sqlite")]
mod ws;
mod error;
mod config;
mod database;
mod scoring;
mod x_api;

#[cfg(feature = "sqlite")]
use ws::client::WebSocketClient;
#[cfg(feature = "sqlite")]
use config::Config;
#[cfg(feature = "sqlite")]
use database::{connection::establish_connection, TokenRepository};
use x_api::XApiClient;
#[cfg(feature = "sqlite")]
use tracing::info;

#[cfg(feature = "sqlite")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    info!("🚀 X Community Bot - Phase 2: WebSocket + Database + Admin Fetch");
    info!("Filter: pump.fun tokens with X Community only");

    // Load configuration
    let config = Config::load()?;
    info!("✓ Configuration loaded");

    // Initialize database connection
    let conn = establish_connection(&config.database.path)?;
    info!("✓ Database connected: {}", config.database.path);

    // Create token repository
    let repo = TokenRepository::new(conn);

    // Create X API client
    let x_api = XApiClient::new(config.x_api.clone());
    info!("✓ X API client initialized");

    // Create and run WebSocket client
    let ws_client = WebSocketClient::new(&repo, &x_api);
    ws_client.run().await?;

    Ok(())
}

#[cfg(feature = "postgres")]
fn main() {
    // PostgreSQL version - to be implemented
    eprintln!("PostgreSQL support: Use the postgres binary instead");
}
