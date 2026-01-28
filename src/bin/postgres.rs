// PostgreSQL version of the bot for production deployment on Railway

use xcommunity_bot::{establish_pool, DbPool, XApiClient, WebSocketClient, XApiConfig, AuthConfig, PgTokenRepository};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    info!("🚀 X Community Bot - PostgreSQL Version");
    info!("Filter: pump.fun tokens with X Community only");

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    info!("✓ Connecting to PostgreSQL...");

    // Initialize database pool
    let pool: DbPool = establish_pool(&database_url).await?;
    info!("✓ Database pool created");

    // Create PostgreSQL repository
    let repo = PgTokenRepository::new(pool.clone());

    // Create X API client
    let x_api_config = XApiConfig {
        base_url: std::env::var("X_API_BASE_URL")
            .unwrap_or_else(|_| "https://x.com/i/api/graphql".to_string()),
        endpoint: std::env::var("X_API_ENDPOINT")
            .unwrap_or_else(|_| "uBpODvS60xZ1q2L88d-W2A/CommunityQuery".to_string()),
        auth: AuthConfig {
            bearer_token: std::env::var("X_API_BEARER_TOKEN")
                .expect("X_API_BEARER_TOKEN environment variable must be set"),
            csrf_token: std::env::var("X_API_CSRF_TOKEN")
                .expect("X_API_CSRF_TOKEN environment variable must be set"),
            cookie: std::env::var("X_API_COOKIE")
                .expect("X_API_COOKIE environment variable must be set"),
        },
    };

    let x_api = XApiClient::new(x_api_config);
    info!("✓ X API client initialized");

    // Create and run WebSocket client
    let ws_client = WebSocketClient::new(&repo, &x_api);
    ws_client.run().await?;

    Ok(())
}
