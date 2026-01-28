use deadpool_postgres::{Pool, Runtime};
use tokio_postgres::NoTls;
use anyhow::Result;

pub type DbPool = Pool;

pub async fn establish_pool(database_url: &str) -> Result<DbPool> {
    let config = deadpool_postgres::Config::from_string(database_url)?;
    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(pool)
}
