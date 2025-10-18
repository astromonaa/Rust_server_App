use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentConfiguration {
    pub shop_id: String,
    pub secret_key: String,
}

impl PaymentConfiguration {
    pub fn load() -> anyhow::Result<Self> {
        let _ = dotenvy::dotenv();

        let settings_reader = Config::builder()
            .add_source(config::Environment::with_prefix("PAYMENT"))
            .build()?;

        let settings = settings_reader.try_deserialize::<PaymentConfiguration>()?;
        Ok(settings)
    }
}