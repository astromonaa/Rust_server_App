use serde::{Deserialize, Serialize};
use anyhow::Result;
use config::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalConfiguration {
    pub client_url: String,
    pub access_secret_key: String,
    pub refresh_secret_key: String,
    pub domain: String,
}

impl GlobalConfiguration {
    pub fn load() -> Result<Self> {
        // Подгрузим .env, если он есть
        let _ = dotenvy::dotenv();

        let settings_reader = Config::builder()
            .add_source(config::Environment::default()) // 🔥 без префикса!
            .build()?;

        let settings = settings_reader.try_deserialize()?;
        Ok(settings)
    }
}
