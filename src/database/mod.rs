#[cfg(feature = "sqlite")]
pub mod connection;
pub mod models;
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "postgres")]
pub mod postgres_repository;
#[cfg(feature = "sqlite")]
pub mod repository;

use crate::ws::message::PoolParams;
use anyhow::Result;

/// Common trait for token repository operations
/// Implemented by both SQLite and PostgreSQL repositories
#[async_trait::async_trait]
pub trait TokenRepositoryTrait: Send + Sync {
    async fn token_exists(&self, pool_address: &str) -> Result<bool>;
    async fn token_has_admin(&self, pool_address: &str) -> Result<bool>;
    async fn upsert_token(&self, pool: &PoolParams, detected_at: i64) -> Result<()>;
    async fn update_admin_username(&self, pool_address: &str, admin: &str) -> Result<()>;
    async fn update_ath_and_score(&self, pool_address: &str, pool: &PoolParams, score: Option<i64>) -> Result<bool>;
    async fn detect_and_update_migration(&self, pool_address: &str, pool: &PoolParams) -> Result<bool>;
    async fn get_token_for_scoring(&self, pool_address: &str) -> Result<Option<(Option<String>, i64, Option<i64>, Option<i64>)>>;
    async fn get_token_score(&self, pool_address: &str) -> Result<Option<i64>>;
    async fn get_admin_for_community(&self, community_id: &str) -> Result<Option<String>>;
}

#[cfg(feature = "sqlite")]
pub use repository::TokenRepository;
#[cfg(feature = "postgres")]
pub use postgres::{establish_pool, DbPool};
#[cfg(feature = "postgres")]
pub use postgres_repository::PgTokenRepository;
