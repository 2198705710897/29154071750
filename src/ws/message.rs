use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WsMessage {
    pub jsonrpc: String,
    pub method: String,
    pub params: PoolParams,
}

#[derive(Debug, Deserialize)]
pub struct PoolParams {
    #[serde(rename = "poolAddress")]
    pub pool_address: String,
    pub chain: String,
    #[serde(rename = "preFactory")]
    pub pre_factory: Option<String>,
    pub factory: String,
    pub router: Option<String>,
    #[serde(rename = "baseToken")]
    pub base_token: String,
    #[serde(rename = "quoteToken")]
    pub quote_token: String,
    #[serde(rename = "curvePercent")]
    pub curve_percent: Option<String>,
    #[serde(rename = "marketCap")]
    pub market_cap: String,
    #[serde(rename = "priceUsd")]
    pub price_usd: String,
    #[serde(rename = "liquidUsd")]
    pub liquid_usd: String,
    #[serde(default)]
    pub report: TradeReport,
    #[serde(rename = "baseTokenInfo")]
    pub base_token_info: BaseTokenInfo,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct BaseTokenInfo {
    pub symbol: String,
    pub name: String,
    #[serde(rename = "logoUri")]
    pub logo_uri: Option<String>,
    #[serde(rename = "totalSupply")]
    pub total_supply: Option<String>,
    #[serde(default)]
    pub social: Option<SocialInfo>,
    pub owner: Option<String>,
    #[serde(rename = "holderCount")]
    pub holder_count: i64,
}

#[derive(Debug, Deserialize, Default)]
pub struct SocialInfo {
    pub twitter: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct TradeReport {
    #[serde(rename = "priceChangePercent", default)]
    pub price_change_percent: String,
    #[serde(rename = "totalVolume", default)]
    pub total_volume: String,
    #[serde(rename = "buyVolume", default)]
    pub buy_volume: String,
    #[serde(rename = "sellVolume", default)]
    pub sell_volume: String,
    #[serde(rename = "tradeCount", default)]
    pub trade_count: i64,
    #[serde(rename = "buyCount", default)]
    pub buy_count: i64,
    #[serde(rename = "sellCount", default)]
    pub sell_count: i64,
}

// Subscription message
#[derive(Debug, Serialize)]
pub struct SubscribeMessage {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: SubscribeParams,
}

#[derive(Debug, Serialize)]
pub struct SubscribeParams {
    pub chain: String,
}
