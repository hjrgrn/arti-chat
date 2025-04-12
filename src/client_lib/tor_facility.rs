use arti_client::{config::TorClientConfigBuilder, DataStream, TorAddr, TorClient};

use crate::client_lib::settings::Settings;

// TODO:
pub async fn build_tor_client_and_connect(
    settings: &Settings,
) -> Result<DataStream, anyhow::Error> {
    let tor_config =
        TorClientConfigBuilder::from_directories(settings.state_dir(), settings.cache_dir())
            .build()
            .map_err(|e| anyhow::anyhow!(e))?;
    let tor_client = TorClient::create_bootstrapped(tor_config)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    let stream = tor_client
        .connect(settings.get_full_onion_address())
        .await?;

    Ok(stream)
}
