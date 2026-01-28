use deadpool_postgres::Pool;
use anyhow::Result;

use crate::ws::message::PoolParams;
use crate::database::TokenRepositoryTrait;

pub struct PgTokenRepository {
    pub pool: Pool,
}

impl PgTokenRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    fn extract_community_id(url: &str) -> Option<String> {
        // Extract ID from https://x.com/i/communities/2016360408018460972
        if let Some(idx) = url.find("x.com/i/communities/") {
            let id_part = &url[idx + 20..];
            Some(id_part.split('/').next().unwrap_or(id_part).to_string())
        } else {
            None
        }
    }

    /// Extract community_id from either twitter or website field
    pub fn extract_community_id_from_pool(pool: &PoolParams) -> Option<String> {
        let social = pool.base_token_info.social.as_ref();

        // Try twitter field first
        if let Some(twitter_url) = social.and_then(|s| s.twitter.as_ref()) {
            if let Some(id) = Self::extract_community_id(twitter_url) {
                return Some(id);
            }
        }

        // Try website field
        if let Some(website_url) = social.and_then(|s| s.website.as_ref()) {
            if let Some(id) = Self::extract_community_id(website_url) {
                return Some(id);
            }
        }

        None
    }
}

#[async_trait::async_trait]
impl TokenRepositoryTrait for PgTokenRepository {
    async fn token_exists(&self, pool_address: &str) -> Result<bool> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(
            "SELECT COUNT(*) FROM tokens WHERE pool_address = $1"
        ).await?;

        let rows = client.query_one(&stmt, &[&pool_address]).await?;
        let count: i64 = rows.get(0);
        Ok(count > 0)
    }

    async fn token_has_admin(&self, pool_address: &str) -> Result<bool> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(
            "SELECT admin_username IS NOT NULL FROM tokens WHERE pool_address = $1"
        ).await?;

        let rows = client.query_opt(&stmt, &[&pool_address]).await?;
        if let Some(row) = rows {
            let has_admin: bool = row.get(0);
            Ok(has_admin)
        } else {
            Ok(false)
        }
    }

    async fn upsert_token(&self, pool: &PoolParams, detected_at: i64) -> Result<()> {
        let client = self.pool.get().await?;

        // Extract community ID from either twitter or website field
        let community_id = Self::extract_community_id_from_pool(pool);

        // Determine platform
        let platform = if pool.factory == "pump" {
            "pump.fun".to_string()
        } else if pool.factory == "pumpamm" && pool.pre_factory.as_deref() == Some("pump") {
            "pump.fun".to_string()
        } else {
            "other".to_string()
        };

        let stmt = client.prepare(
            "INSERT INTO tokens (
                pool_address, base_token, token_symbol, token_name,
                community_id, twitter_url, market_cap, price_usd,
                liquid_usd, holder_count, created_at, detected_at,
                total_volume, buy_count, sell_count, platform,
                last_updated
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT(pool_address) DO UPDATE SET
                market_cap = EXCLUDED.market_cap,
                price_usd = EXCLUDED.price_usd,
                liquid_usd = EXCLUDED.liquid_usd,
                holder_count = EXCLUDED.holder_count,
                total_volume = EXCLUDED.total_volume,
                buy_count = EXCLUDED.buy_count,
                sell_count = EXCLUDED.sell_count,
                last_updated = EXCLUDED.last_updated"
        ).await?;

        client.execute(
            &stmt,
            &[
                &pool.pool_address,
                &pool.base_token,
                &pool.base_token_info.symbol,
                &pool.base_token_info.name,
                &community_id,
                &pool.base_token_info.social.as_ref().and_then(|s| s.twitter.as_ref()),
                &pool.market_cap.parse::<f64>().ok(),
                &pool.price_usd.parse::<f64>().ok(),
                &pool.liquid_usd.parse::<f64>().ok(),
                &pool.base_token_info.holder_count,
                &pool.created_at,
                &detected_at,
                &pool.report.total_volume.parse::<f64>().ok(),
                &pool.report.buy_count,
                &pool.report.sell_count,
                &platform,
                &detected_at,
            ],
        ).await?;

        Ok(())
    }

    async fn update_admin_username(&self, pool_address: &str, admin: &str) -> Result<()> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(
            "UPDATE tokens SET admin_username = $1 WHERE pool_address = $2"
        ).await?;

        client.execute(&stmt, &[&admin, &pool_address]).await?;
        Ok(())
    }

    async fn update_ath_and_score(
        &self,
        pool_address: &str,
        pool: &PoolParams,
        score: Option<i64>,
    ) -> Result<bool> {
        let client = self.pool.get().await?;

        let current_mc = pool.market_cap.parse::<f64>().ok();

        // Get current ATH
        let stmt = client.prepare(
            "SELECT ath_market_cap, ath_detected_at FROM tokens WHERE pool_address = $1"
        ).await?;

        let rows = client.query_opt(&stmt, &[&pool_address]).await?;
        let (current_ath, current_ath_detected) = if let Some(row) = rows {
            let ath: Option<String> = row.get(0);
            let detected: Option<i64> = row.get(1);
            (ath, detected)
        } else {
            (None, None)
        };

        // Check if new ATH
        let mut new_ath = current_ath.clone();
        let mut new_ath_detected = current_ath_detected;
        let mut new_ath_detected_bool = false;

        if let Some(mc) = current_mc {
            let current_ath_value = current_ath.as_deref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            if mc > current_ath_value {
                new_ath = Some(pool.market_cap.clone());
                new_ath_detected = Some(chrono::Utc::now().timestamp());
                new_ath_detected_bool = true;
            }
        }

        // Update database with ATH and score
        let stmt = client.prepare(
            "UPDATE tokens SET
                ath_market_cap = $1,
                ath_detected_at = $2,
                token_score = $3
            WHERE pool_address = $4"
        ).await?;

        client.execute(
            &stmt,
            &[&new_ath, &new_ath_detected, &score, &pool_address],
        ).await?;

        Ok(new_ath_detected_bool)
    }

    async fn detect_and_update_migration(&self, pool_address: &str, pool: &PoolParams) -> Result<bool> {
        // Check if this is a migration event (factory changed to pumpamm)
        if pool.factory != "pumpamm" {
            return Ok(false);
        }

        let client = self.pool.get().await?;

        // Get current migration status
        let stmt = client.prepare(
            "SELECT is_migrated, migrated_at FROM tokens WHERE pool_address = $1"
        ).await?;

        let rows = client.query_opt(&stmt, &[&pool_address]).await?;
        let (is_migrated, _migrated_at) = if let Some(row) = rows {
            let migrated: i64 = row.get(0);
            let at: Option<i64> = row.get(1);
            (migrated, at)
        } else {
            (0i64, None)
        };

        // If already migrated, don't update again
        if is_migrated == 1 {
            return Ok(false);
        }

        // Update migration status
        let now = chrono::Utc::now().timestamp();
        let market_cap_at_migration = pool.market_cap.parse::<f64>().ok();

        let stmt = client.prepare(
            "UPDATE tokens SET
                is_migrated = 1,
                migrated_at = $1,
                market_cap_at_migration = $2
            WHERE pool_address = $3"
        ).await?;

        client.execute(&stmt, &[&now, &market_cap_at_migration, &pool_address]).await?;

        Ok(true)
    }

    async fn get_token_for_scoring(&self, pool_address: &str) -> Result<Option<(Option<String>, i64, Option<i64>, Option<i64>)>> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(
            "SELECT ath_market_cap, is_migrated, migrated_at, created_at FROM tokens WHERE pool_address = $1"
        ).await?;

        let rows = client.query_opt(&stmt, &[&pool_address]).await?;

        if let Some(row) = rows {
            let result: (Option<String>, i64, Option<i64>, Option<i64>) = (
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3),
            );
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    async fn get_token_score(&self, pool_address: &str) -> Result<Option<i64>> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(
            "SELECT token_score FROM tokens WHERE pool_address = $1"
        ).await?;

        let rows = client.query_opt(&stmt, &[&pool_address]).await?;

        if let Some(row) = rows {
            let score: Option<i64> = row.get(0);
            Ok(score)
        } else {
            Ok(None)
        }
    }

    async fn get_admin_for_community(&self, community_id: &str) -> Result<Option<String>> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(
            "SELECT admin_username FROM tokens
             WHERE community_id = $1 AND admin_username IS NOT NULL
             LIMIT 1"
        ).await?;

        let rows = client.query_opt(&stmt, &[&community_id]).await?;

        if let Some(row) = rows {
            let admin: Option<String> = row.get(0);
            Ok(admin)
        } else {
            Ok(None)
        }
    }
}
