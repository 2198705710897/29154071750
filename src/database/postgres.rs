use deadpool_postgres::Pool;
use tokio_postgres::NoTls;
use anyhow::Result;
use std::str::FromStr;

pub type DbPool = Pool;

pub async fn establish_pool(database_url: &str) -> Result<DbPool> {
    // Parse the database URL using tokio_postgres Config
    let pg_config = tokio_postgres::Config::from_str(database_url)?;

    // Create the pool with the parsed config
    let manager = deadpool_postgres::Manager::new(pg_config, NoTls);
    let pool = Pool::builder(manager)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to create pool: {}", e))?;

    Ok(pool)
}
