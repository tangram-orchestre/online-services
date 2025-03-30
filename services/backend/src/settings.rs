use config::Environment;

use config::Config;

use config::ConfigError;

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub host: String,

    pub otlp_endpoint: String,
    pub otlp_service_name: String,

    pub cors_origins: String,

    pub altcha_secret: String,

    pub smtp_host: String,
    pub smtp_name: Option<String>,
    pub smtp_password: Option<String>,
    pub postgres_url: String,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(Environment::with_prefix("app"))
            .build()?
            .try_deserialize()
    }
}
