use clap::Parser;
use mesh_core::mesh::MeshMessage;
use mesh_core::utils::{generate_certificate_authority, generate_node_certs};
use mesh_core::{link::Link, types::args::Args, wifi::WifiQuicLink};
use prost::Message;
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
        Ok(()) => log::info!("Logger successfully initialized for Mesh Core!"),
        Err(e) => log::error!("Logger couldn't be initialized for Mesh Core: {}", e),
    }
    log::info!("Mesh Core Initialized!");

    let (ca_cert, ca_issuer) = generate_certificate_authority();
    let (_node1_cert, _node1_key) = generate_node_certs(&ca_issuer, "Node 1");
    let (_node2_cert, _node2_key) = generate_node_certs(&ca_issuer, "Node 2");

    let node1 = WifiQuicLink::new(
        "127.0.0.1:8000",
        &[ca_cert.der().clone().into_owned()],
        "node1",
        &ca_issuer,
    )?;
    let n1 = node1.clone();
    let node2 = WifiQuicLink::new(
        "127.0.0.1:8001",
        &[ca_cert.der().clone().into_owned()],
        "node2",
        &ca_issuer,
    )?;
    let n2 = node2.clone();

    // node1 server
    tokio::spawn(async move {
        let connection = node1.clone().accept().await.unwrap();
        loop {
            match connection.receive().await {
                Ok(data) => {
                    log::info!("Raw bytes received: {:?}", data);
                    let msg = MeshMessage::decode(&data[..]).unwrap();
                    log::info!("Decoded message: {:?}", msg);
                }
                Err(e) => {
                    log::error!("Error while receiving message from client: {}", e);
                }
            }
        }
    });

    tokio::spawn(async move {
        let connection = node2.clone().accept().await.unwrap();
        loop {
            match connection.receive().await {
                Ok(data) => {
                    let end = data.iter().position(|c| *c == 0).unwrap_or(data.len());
                    let slice = &data[..end];
                    let msg = MeshMessage::decode(&slice[..]).unwrap();
                    log::info!("Decoded message: {:?}", msg);
                }
                Err(e) => {
                    log::error!("Error while receiving message from client: {}", e);
                }
            }
        }
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // connecting node1 and node2
    let node1_connection = n1.dial("127.0.0.1:8001").await?;
    let message = MeshMessage {
        sender: "Node1".to_string(),
        content: "Hello Node2".to_string(),
    };
    let mut buf = Vec::new();
    message.encode(&mut buf)?;
    // send data from node1 to node2
    match node1_connection.send(&buf).await {
        Ok(_) => log::info!("Data sent successfully: {:?}", buf),
        Err(e) => log::error!("Error: {}", e),
    }

    // connecting node1 and node2
    let node2_connection = n2.dial("127.0.0.1:8000").await?;

    let message = MeshMessage {
        sender: "Node2".to_string(),
        content: "Hello Node1".to_string(),
    };
    let mut buf = Vec::new();
    message.encode(&mut buf)?;
    // send data from node1 to node2
    match node2_connection.send(&buf).await {
        Ok(_) => log::info!("Data sent successfully: {:?}", buf),
        Err(e) => log::error!("Error: {}", e),
    }

    Ok(())
}
