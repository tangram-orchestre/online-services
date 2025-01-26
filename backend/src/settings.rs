use config::Environment;

use config::Config;

use config::ConfigError;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Settings {
    pub(crate) cors_origins: String,
}

impl Settings {
    pub(crate) fn load() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(Environment::with_prefix("app"))
            .build()?
            .try_deserialize()
    }
}
