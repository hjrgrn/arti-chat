use std::str::FromStr;

use arti_client::{
    config::{onion_service::OnionServiceConfigBuilder, TorClientConfigBuilder},
    TorClient,
};

use futures::{Stream, StreamExt};
use tor_hsservice::{HsNickname, RendRequest};

use crate::server_lib::settings::Settings;

pub async fn launch_service(
    settings: &Settings,
) -> Result<impl Stream<Item = RendRequest>, anyhow::Error> {
    let tor_config =
        TorClientConfigBuilder::from_directories(settings.state_dir(), settings.cache_dir())
            .build()
            .map_err(|e| anyhow::anyhow!(e))?;
    let tor_client = TorClient::create_bootstrapped(tor_config)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    let svc_config = OnionServiceConfigBuilder::default()
        .nickname(HsNickname::from_str("arti-chat-server").map_err(|e| anyhow::anyhow!(e))?)
        .build()
        .map_err(|e| anyhow::anyhow!(e))?;

    let (service, request_stream) = tor_client
        .launch_onion_service(svc_config)
        .map_err(|e| anyhow::anyhow!(e))?;
    if let Some(addr) = service.onion_address() {
        println!("Address: {}", addr);
    } else {
        return Err(anyhow::anyhow!(
            "Failed to identify onion address of the service"
        ));
    }

    // Wait until the service is believed to be fully reachable.
    while let Some(status) = service.status_events().next().await {
        if status.state().is_fully_reachable() {
            println!("arti-chat server is fully reachable");
            break;
        }
    }

    Ok(request_stream)
}
