use reqwest::Client;
use anyhow::Result;
use uuid::Uuid;

use super::models::{XApiConfig, XApiResponse};

pub struct XApiClient {
    client: Client,
    config: XApiConfig,
}

impl XApiClient {
    pub fn new(config: XApiConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .http1_only()
            .build()
            .unwrap();
        Self { client, config }
    }

    /// Fetch the admin username for a community
    /// Returns Some(username) if admin found, None if not found or API error
    pub async fn fetch_community_admin(&self, community_id: &str) -> Result<Option<String>> {
        // Debug: Log first 100 chars of bearer token and cookie
        tracing::debug!("Bearer token (first 100): {}...", &self.config.auth.bearer_token[..self.config.auth.bearer_token.len().min(100)]);
        tracing::debug!("Cookie (first 200): {}...", &self.config.auth.cookie[..self.config.auth.cookie.len().min(200)]);

        let url = format!("{}/{}", self.config.base_url, self.config.endpoint);

        // Query parameters for CommunityQuery endpoint
        let variables = format!(r#"{{"communityId":"{}"}}"#, community_id);
        let features = r#"{"c9s_list_members_action_api_enabled":false,"c9s_superc9s_indication_enabled":false}"#;

        let referer = format!("https://x.com/i/communities/{}", community_id);

        let response = self.client
            .get(&url)
            .query(&[("variables", variables)])
            .query(&[("features", features)])
            .header("authorization", format!("Bearer {}", self.config.auth.bearer_token))
            .header("content-type", "application/json")
            .header("x-csrf-token", &self.config.auth.csrf_token)
            .header("cookie", &self.config.auth.cookie)
            .header("x-twitter-active-user", "yes")
            .header("x-twitter-auth-type", "OAuth2Session")
            .header("x-twitter-client-language", "en")
            .header("accept", "*/*")
            .header("accept-language", "en-US,en;q=0.9")
            .header("referer", &referer)
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36")
            .header("sec-ch-ua", r#""Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24""#)
            .header("sec-ch-ua-arch", r#""x86""#)
            .header("sec-ch-ua-bitness", r#""64""#)
            .header("sec-ch-ua-full-version", r#""143.0.7499.170""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-model", r#"""""#)
            .header("sec-ch-ua-platform", r#""Windows""#)
            .header("sec-ch-ua-platform-version", r#""10.0.0""#)
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("priority", "u=1, i")
            .header("x-client-transaction-id", Uuid::new_v4().to_string())
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status = resp.status();
                if !status.is_success() {
                    // Try to read the error response body
                    let body = resp.text().await.unwrap_or_else(|_| "<unable to read body>".to_string());
                    tracing::warn!("X API returned status: {} | Body: {}", status, body);
                    return Ok(None);
                }

                match resp.json::<XApiResponse>().await {
                    Ok(api_response) => {
                        // Admin is the creator: data.communityResults.result.creator_results.result.core.screen_name
                        Ok(Some(api_response.data.community_results.result.creator_results.result.core.screen_name))
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse X API response: {}", e);
                        Ok(None)
                    }
                }
            }
            Err(e) => {
                tracing::warn!("X API request failed: {}", e);
                Ok(None)
            }
        }
    }
}
