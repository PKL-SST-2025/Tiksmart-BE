// File: be-api/src/config/mod.rs

use once_cell::sync::Lazy;
use serde::Deserialize;

// 1. The struct that will hold all our application's configuration.
// `Deserialize` allows us to read the config from the environment.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_address: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub frontend_origin: String,
    pub rate_limit_requests: u32,     
    pub rate_limit_period_seconds: u64, 
    pub csrf_secret: String,
    pub env: String,

    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
}

// 2. A function to load the configuration from the environment.
impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            // Tell it to look for environment variables prefixed with "APP_"
            // and that `_` separates nested fields.
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}

// 3. Create a global, static, lazily-initialized configuration.
// `Lazy` from `once_cell` ensures that `Config::from_env()` is called
// exactly once, the first time `CONFIG` is accessed.
// If it fails, the application will panic at startup, which is what we want.
pub static CONFIG: Lazy<Config> =
    Lazy::new(|| Config::from_env().expect("Failed to load configuration from environment"));