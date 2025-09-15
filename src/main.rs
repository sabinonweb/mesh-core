use clap::Parser;
use mesh_core::{
    configure::{make_client_endpoint, make_server_endpoint},
    types::args::Args,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
async fn task(_name: &str, interval: u64) {
    loop {
        println!("Task {} tick", interval);
        sleep(Duration::from_secs(interval)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let arguments = Args::parse();

    match log4rs::init_file(&arguments.log_config, Default::default()) {
        Ok(()) => log::info!("Logger successfully initialized for Plant Go!"),
        Err(e) => log::error!("Logger couldn't be initialized for Plant Go: {}", e),
    }
    log::info!("Mesh Core Initialized!");

    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);
    let (endpoint, certificates) = make_server_endpoint(server_addr).unwrap();
    let ep = endpoint.clone();

    tokio::spawn(async move {
        // incoming connections are now accepted, hanshake is initiated
        // returns none if the endpoint is closed
        if let Some(incoming_connection) = ep.accept().await {
            match incoming_connection.await {
                Ok(connection) => log::info!(
                    "Connection established to remote server {}",
                    connection.remote_address()
                ),
                Err(e) => log::error!(
                    "Error occured while trying to establish a connection to the remote server {}",
                    e
                ),
            }
        }
    });

    let client_endpoint =
        make_client_endpoint(&[&certificates], "0.0.0.0:0".parse().unwrap()).unwrap();
    // iniitates handshake to server and awaits it and returns a connection
    let connection = client_endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    log::info!(
        "Client Connection established {}",
        connection.remote_address()
    );

    // for graceful cleaning
    endpoint.wait_idle().await;

    Ok(())
}
