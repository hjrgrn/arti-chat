use std::{env, process::exit};

use arti_client::{
    config::{onion_service::OnionServiceConfigBuilder, TorClientConfigBuilder},
    TorClient, TorClientConfig,
};
use lib::{
    server_lib::{
        self, administration::server_commands_wrapper, settings::get_settings,
        ConnHandlerIdRecordMsg,
    },
    shared_lib::{display_output, graceful_shutdown::handling_sigint, OutputMsg, StdinRequest},
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::SecretString;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[tokio::main]
pub async fn main() {
    // NOTE: for the time being it will be possible to abtain the shared secret only from an
    // environment variable `ASYNC_CHAT_SECRET`
    let shared_secret = SecretString::from(env::var("ASYNC_CHAT_SECRET").expect("Failed to obtain the shared secret, write that into the environment variable \"ASYNC_CHAT_SECRET\""));

    let sub = get_subscriber("TcpChatServer".into(), "warn".into(), std::io::stdout);
    init_subscriber(sub);
    let settings = match get_settings() {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to read settings:\n{:?}", e);
            return;
        }
    };

    let tor_config =
        TorClientConfigBuilder::from_directories(settings.state_dir(), settings.cache_dir())
            .build()
            .expect("Failed to build TorClientConfig");
    let tor_client = TorClient::create_bootstrapped(tor_config)
        .await
        .expect("Failed to create TorClient");

    let svc_config = OnionServiceConfigBuilder::default()
        .nickname(
            "arti-chat-server"
                .parse()
                .expect("Failed to parse server nickname, this should not happen"),
        )
        .build()
        .expect("Failed to build onion service config");

    let (service, request_stream) = tor_client
        .launch_onion_service(svc_config)
        .expect("Failed to launch onion service");
    println!(
        "Address: {}",
        service
            .onion_address()
            .expect("Failed to identify onion address of the service")
    );

    exit(19);

    // Cancellation token for graceful shutdown
    let ctoken = CancellationToken::new();

    // spawn the function that allow the output of the server to be displayed
    let (output_tx, output_rx) = mpsc::channel::<OutputMsg>(10);
    tokio::spawn(display_output(output_rx, ctoken.clone()));

    // spawn the function that handles graceful shutdown
    tokio::spawn(handling_sigint(ctoken.clone(), output_tx.clone()));

    // spawn the function that allow the admin to communicate with the server
    let (con_hand_id_tx, con_hand_id_rx) = mpsc::channel::<ConnHandlerIdRecordMsg>(10);
    let (stdin_req_tx, stdin_req_rx) = mpsc::channel::<StdinRequest>(10);
    tokio::spawn(server_commands_wrapper(
        con_hand_id_tx.clone(),
        stdin_req_rx,
        output_tx.clone(),
        ctoken.clone(),
    ));

    server_lib::run_wrapper(
        settings,
        con_hand_id_tx,
        con_hand_id_rx,
        output_tx,
        stdin_req_tx,
        ctoken,
        shared_secret,
    )
    .await
}
