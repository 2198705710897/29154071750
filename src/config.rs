use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub x_api: XApiConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct XApiConfig {
    pub base_url: String,
    pub endpoint: String,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub bearer_token: String,
    pub csrf_token: String,
    pub cookie: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;

        let config: Config = settings.try_deserialize()?;
        Ok(config)
    }
}
