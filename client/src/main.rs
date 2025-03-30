mod config;

use crate::config::load_config;
use axum::{routing::get, Router};
use config::Config;
use serde::Serialize;
use tfhe::{generate_keys, ConfigBuilder, PublicKey};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting client");

    let config = load_config().unwrap();

    let app = Router::new().route("/health", get(|| async { "Hello, world!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub fn build_vault_client(config: &Config) -> eyre::Result<VaultClient> {
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(config.vault_addr.clone())
            .token(config.vault_token.clone())
            .build()
            .unwrap(),
    )?;
    Ok(client)
}
