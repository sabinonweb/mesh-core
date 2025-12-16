use std::error::Error;

use rustmesh::link::{wifi::WifiLink, Link};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let binding_addr = "192.168.110.134:8080";
    let remote_addr = "192.168.110.18:8081";
    let wifi_link = WifiLink::new(binding_addr, remote_addr).await.unwrap();

    wifi_link
        .send(b"sabinonwebbabybyby from macbook babby come on kr$na")
        .await
        .unwrap();
    println!("Message Sent!");

    if let Some(data) = wifi_link.recv().await.unwrap() {
        println!("Received: {:?}", String::from_utf8_lossy(&data));
    }

    Ok(())
}
