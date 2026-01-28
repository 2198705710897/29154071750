use serde::Deserialize;

// Re-export from config module
pub use crate::config::XApiConfig;

/// X API GraphQL response structure for CommunityQuery
#[derive(Debug, Deserialize)]
pub struct XApiResponse {
    pub data: CommunityResults,
}

#[derive(Debug, Deserialize)]
pub struct CommunityResults {
    #[serde(rename = "communityResults")]
    pub community_results: CommunityResult,
}

#[derive(Debug, Deserialize)]
pub struct CommunityResult {
    pub result: Community,
}

#[derive(Debug, Deserialize)]
pub struct Community {
    #[serde(rename = "creator_results")]
    pub creator_results: CreatorResult,
}

#[derive(Debug, Deserialize)]
pub struct CreatorResult {
    pub result: CreatorUser,
}

#[derive(Debug, Deserialize)]
pub struct CreatorUser {
    pub core: UserCore,
}

#[derive(Debug, Deserialize)]
pub struct UserCore {
    #[serde(rename = "screen_name")]
    pub screen_name: String,
}
