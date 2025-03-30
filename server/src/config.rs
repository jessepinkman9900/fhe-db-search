use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub vault_addr: String,
    pub vault_token: String,
}

pub fn load_config() -> eyre::Result<Config> {
    match envy::prefixed("APP_").from_env::<Config>() {
        Ok(config) => {
            log::info!("Loaded config: {:#?}", config);
            Ok(config)
        }
        Err(e) => {
            log::error!("Failed to load config: {}", e);
            Err(eyre::eyre!("Failed to load config: {}", e))
        }
    }
}
