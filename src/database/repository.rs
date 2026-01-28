use rusqlite::{params, Connection, OptionalExtension};
use anyhow::Result;

use crate::ws::message::PoolParams;

pub struct TokenRepository {
    pub conn: Connection,
}

impl TokenRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn token_exists(&self, pool_address: &str) -> Result<bool> {
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) as count FROM tokens WHERE pool_address = ?1"
        )?;

        let count: i64 = stmt.query_row(params![pool_address], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn token_has_admin(&self, pool_address: &str) -> Result<bool> {
        let mut stmt = self.conn.prepare(
            "SELECT admin_username IS NOT NULL as has_admin FROM tokens WHERE pool_address = ?1"
        )?;

        let has_admin: bool = stmt.query_row(params![pool_address], |row| row.get(0)).unwrap_or(false);
        Ok(has_admin)
    }

    pub fn upsert_token(&self, pool: &PoolParams, detected_at: i64) -> Result<()> {
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

        self.conn.execute(
            "INSERT INTO tokens (
                pool_address, base_token, token_symbol, token_name,
                community_id, twitter_url, market_cap, price_usd,
                liquid_usd, holder_count, created_at, detected_at,
                total_volume, buy_count, sell_count, platform,
                last_updated
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)
            ON CONFLICT(pool_address) DO UPDATE SET
                market_cap = excluded.market_cap,
                price_usd = excluded.price_usd,
                liquid_usd = excluded.liquid_usd,
                holder_count = excluded.holder_count,
                total_volume = excluded.total_volume,
                buy_count = excluded.buy_count,
                sell_count = excluded.sell_count,
                last_updated = excluded.last_updated",
            params![
                pool.pool_address,
                pool.base_token,
                pool.base_token_info.symbol,
                pool.base_token_info.name,
                community_id,
                pool.base_token_info.social.as_ref().and_then(|s| s.twitter.as_ref()),
                pool.market_cap.parse::<f64>().ok(),
                pool.price_usd.parse::<f64>().ok(),
                pool.liquid_usd.parse::<f64>().ok(),
                pool.base_token_info.holder_count,
                pool.created_at,
                detected_at,
                pool.report.total_volume.parse::<f64>().ok(),
                pool.report.buy_count,
                pool.report.sell_count,
                platform,
                detected_at,
            ],
        )?;

        Ok(())
    }

    pub fn update_admin_username(&self, pool_address: &str, admin: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE tokens SET admin_username = ?1 WHERE pool_address = ?2",
            params![admin, pool_address],
        )?;
        Ok(())
    }

    pub fn update_ath_and_score(
        &self,
        pool_address: &str,
        pool: &PoolParams,
        score: Option<i64>,
    ) -> Result<bool> {
        let current_mc = pool.market_cap.parse::<f64>().ok();

        // Get current ATH
        let (current_ath, current_ath_detected): (Option<String>, Option<i64>) = self.conn.query_row(
            "SELECT ath_market_cap, ath_detected_at FROM tokens WHERE pool_address = ?1",
            params![pool_address],
            |row| Ok((row.get(0)?, row.get(1)?))
        ).unwrap_or((None, None));

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
        self.conn.execute(
            "UPDATE tokens SET
                ath_market_cap = ?1,
                ath_detected_at = ?2,
                token_score = ?3
            WHERE pool_address = ?4",
            params![
                new_ath,
                new_ath_detected,
                score,
                pool_address,
            ],
        )?;

        Ok(new_ath_detected_bool)
    }

    pub fn detect_and_update_migration(&self, pool_address: &str, pool: &PoolParams) -> Result<bool> {
        // Check if this is a migration event (factory changed to pumpamm)
        if pool.factory != "pumpamm" {
            return Ok(false);
        }

        // Get current migration status
        let (is_migrated, _migrated_at): (i64, Option<i64>) = self.conn.query_row(
            "SELECT is_migrated, migrated_at FROM tokens WHERE pool_address = ?1",
            params![pool_address],
            |row| Ok((row.get(0)?, row.get(1)?))
        ).unwrap_or((0, None));

        // If already migrated, don't update again
        if is_migrated == 1 {
            return Ok(false);
        }

        // Update migration status
        let now = chrono::Utc::now().timestamp();
        let market_cap_at_migration = pool.market_cap.parse::<f64>().ok();

        self.conn.execute(
            "UPDATE tokens SET
                is_migrated = 1,
                migrated_at = ?1,
                market_cap_at_migration = ?2
            WHERE pool_address = ?3",
            params![now, market_cap_at_migration, pool_address],
        )?;

        Ok(true)
    }

    pub fn get_token_for_scoring(&self, pool_address: &str) -> Result<Option<(Option<String>, i64, Option<i64>, Option<i64>)>> {
        let result = self.conn.query_row(
            "SELECT ath_market_cap, is_migrated, migrated_at, created_at FROM tokens WHERE pool_address = ?1",
            params![pool_address],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        ).optional()?;

        Ok(result)
    }

    pub fn get_token_score(&self, pool_address: &str) -> Result<Option<i64>> {
        let mut stmt = self.conn.prepare(
            "SELECT token_score FROM tokens WHERE pool_address = ?1"
        )?;

        let score = stmt.query_row(params![pool_address], |row| row.get(0)).optional()?;
        Ok(score.flatten())
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

    /// Check if we already have an admin username for a given community_id
    /// Returns Some(admin_username) if found, None if not
    pub fn get_admin_for_community(&self, community_id: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT admin_username FROM tokens
             WHERE community_id = ?1 AND admin_username IS NOT NULL
             LIMIT 1"
        )?;

        let admin = stmt.query_row(params![community_id], |row| row.get(0))
            .optional()?;

        Ok(admin.flatten())
    }
}
