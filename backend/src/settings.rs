use config::Environment;

use config::Config;

use config::ConfigError;

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub cors_origins: String,
    pub altcha_secret: String,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(Environment::with_prefix("app"))
            .build()?
            .try_deserialize()
    }
}
