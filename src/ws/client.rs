use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};
use uuid::Uuid;
use anyhow::Result;
use chrono::Utc;

use crate::ws::{message::{SubscribeMessage, SubscribeParams, WsMessage}, filter::should_accept_message};
use crate::database::TokenRepository;
use crate::x_api::XApiClient;
use crate::scoring::TokenScorer;
use crate::error::AppError;

const WS_URL: &str = "wss://ws.mevx.io/api/v1/ws";

/// Format market cap string to a readable number
fn format_mc(mc_str: &str) -> String {
    if let Ok(mc) = mc_str.parse::<f64>() {
        if mc >= 1_000_000.0 {
            format!("{:.0}", mc)
        } else if mc >= 1_000.0 {
            format!("{:.2}", mc)
        } else {
            format!("{:.4}", mc)
        }
    } else {
        mc_str.to_string()
    }
}

pub struct WebSocketClient<'a> {
    repo: &'a TokenRepository,
    x_api: &'a XApiClient,
    bot_start_time: i64,
}

impl<'a> WebSocketClient<'a> {
    pub fn new(repo: &'a TokenRepository, x_api: &'a XApiClient) -> Self {
        let bot_start_time = Utc::now().timestamp();
        tracing::info!("🕐 Bot started at: {}", bot_start_time);
        Self { repo, x_api, bot_start_time }
    }

    pub async fn run(&self) -> Result<(), AppError> {
        let url = WS_URL;
        tracing::info!("Connecting to WebSocket: {}", url);

        let (ws_stream, _) = connect_async(url).await
            .map_err(|e| AppError::WebSocketConnection(e.to_string()))?;

        tracing::info!("✓ Connected to WebSocket");

        let (mut write, mut read) = ws_stream.split();

        // Send subscription message
        let sub_msg = SubscribeMessage {
            jsonrpc: "2.0".to_string(),
            id: Uuid::new_v4().to_string(),
            method: "subscribeFlashPool".to_string(),
            params: SubscribeParams {
                chain: "sol".to_string(),
            },
        };

        let sub_json = serde_json::to_string(&sub_msg)
            .map_err(|e| AppError::Serialization(e.to_string()))?;

        write.send(Message::Text(sub_json)).await
            .map_err(|e| AppError::WebSocketSend(e.to_string()))?;

        tracing::info!("✓ Subscribed to flash pool updates");
        tracing::info!("⏱ Only accepting tokens created after bot start time");

        // Clone references for use in async handler
        let repo = self.repo;
        let x_api = self.x_api;
        let bot_start_time = self.bot_start_time;

        // Listen for messages
        while let Some(msg_result) = read.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    if let Err(e) = Self::handle_message(&text, repo, x_api, bot_start_time).await {
                        tracing::warn!("Failed to handle message: {}", e);
                    }
                }
                Ok(Message::Ping(data)) => {
                    write.send(Message::Pong(data)).await
                        .map_err(|e| AppError::WebSocketSend(e.to_string()))?;
                }
                Ok(Message::Close(_)) => {
                    tracing::warn!("WebSocket closed by server");
                    break;
                }
                Err(e) => {
                    tracing::error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    async fn handle_message(
        text: &str,
        repo: &TokenRepository,
        x_api: &XApiClient,
        bot_start_time: i64,
    ) -> Result<()> {
        // Parse JSON-RPC message
        let msg: WsMessage = match serde_json::from_str(text) {
            Ok(m) => m,
            Err(e) => {
                tracing::debug!("Failed to parse message (not a pool update): {} | Message: {:.100}...", e, text);
                return Ok(()); // Not an error - just a different message type
            }
        };

        // Check if it's a pool update
        if msg.method == "subscribeFlashPool" {
            let pool_address = &msg.params.pool_address;

            // Check if token is already in our database AND has an admin
            let is_tracked = repo.token_exists(pool_address)?;
            let has_admin = repo.token_has_admin(pool_address)?;
            let should_track = is_tracked && has_admin;

            // Apply filters: accept if (already tracked WITH admin) OR (is pump.fun + has community + created after bot start)
            if should_track || should_accept_message(&msg.params, bot_start_time) {
                let detected_at = chrono::Utc::now().timestamp();
                let is_new = !is_tracked;

                // Insert/update token in database
                repo.upsert_token(&msg.params, detected_at)?;

                // Check for migration (factory changed to pumpamm)
                let just_migrated = repo.detect_and_update_migration(pool_address, &msg.params)?;

                // Extract community_id from twitter URL
                if let Some(community_id) = extract_community_id(&msg.params) {
                    // Check if we already have an admin for this community (from any previous token)
                    let cached_admin = repo.get_admin_for_community(&community_id).ok().flatten();

                    if let Some(admin_username) = cached_admin {
                        // We already know this admin - just update the token
                        if !has_admin {
                            repo.update_admin_username(pool_address, &admin_username)?;
                        }
                        if is_new {
                            tracing::info!(
                                "🆕 NEW TOKEN: {} by @{} (cached) | MC: ${}",
                                msg.params.base_token_info.symbol,
                                admin_username,
                                format_mc(&msg.params.market_cap)
                            );
                        }
                    } else if is_new || !has_admin {
                        // New token OR existing token without admin AND no cached admin - fetch from API
                        if let Ok(Some(admin_username)) = x_api.fetch_community_admin(&community_id).await {
                            repo.update_admin_username(pool_address, &admin_username)?;
                            tracing::info!(
                                "{} {} by @{} | MC: ${}",
                                if is_new { "🆕 NEW TOKEN" } else { "✅ ADMIN FETCHED" },
                                msg.params.base_token_info.symbol,
                                admin_username,
                                format_mc(&msg.params.market_cap)
                            );
                        }
                    }

                    if just_migrated {
                        tracing::info!(
                            "🔄 MIGRATED: {} | MC: ${}",
                            msg.params.base_token_info.symbol,
                            format_mc(&msg.params.market_cap)
                        );
                    }
                }

                // Get token data for scoring
                if let Ok(Some((ath, is_migrated, migrated_at, created_at))) =
                    repo.get_token_for_scoring(pool_address) {
                    // Calculate score
                    let score = TokenScorer::calculate_score(
                        ath.as_deref(),
                        is_migrated == 1,
                        migrated_at,
                        created_at,
                    );

                    // Update ATH and score in database
                    if repo.update_ath_and_score(pool_address, &msg.params, score)? {
                        // Log only when new ATH is detected
                        tracing::info!(
                            "📈 NEW ATH: {} | {} | MC: ${}",
                            msg.params.base_token_info.symbol,
                            msg.params.base_token,
                            format_mc(&msg.params.market_cap)
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

fn extract_community_id(pool: &crate::ws::message::PoolParams) -> Option<String> {
    let social = pool.base_token_info.social.as_ref();

    // Helper to extract ID from URL
    let extract_from_url = |url: &str| -> Option<String> {
        if let Some(idx) = url.find("x.com/i/communities/") {
            let id_part = &url[idx + 20..];
            Some(id_part.split('/').next().unwrap_or(id_part).to_string())
        } else {
            None
        }
    };

    // Try twitter field first
    if let Some(twitter_url) = social.and_then(|s| s.twitter.as_ref()) {
        if let Some(id) = extract_from_url(twitter_url) {
            return Some(id);
        }
    }

    // Try website field
    if let Some(website_url) = social.and_then(|s| s.website.as_ref()) {
        if let Some(id) = extract_from_url(website_url) {
            return Some(id);
        }
    }

    None
}
