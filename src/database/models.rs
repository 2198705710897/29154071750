use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub pool_address: String,
    pub base_token: String,
    pub token_symbol: String,
    pub token_name: String,
    pub community_id: Option<String>,
    pub admin_username: Option<String>,
    pub twitter_url: Option<String>,
    pub market_cap: Option<f64>,
    pub price_usd: Option<f64>,
    pub liquid_usd: Option<f64>,
    pub holder_count: i64,
    pub created_at: i64,
    pub detected_at: i64,
    pub total_volume: Option<f64>,
    pub buy_count: i64,
    pub sell_count: i64,
    pub ath_market_cap: Option<String>,
    pub ath_detected_at: Option<i64>,
    pub is_migrated: i64,
    pub migrated_at: Option<i64>,
    pub market_cap_at_migration: Option<f64>,
    pub token_score: Option<i64>,
    pub platform: String,
    pub last_updated: i64,
}
