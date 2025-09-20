use mesh_core::types::peer::PeerID;
use mesh_core::udpsocket::server;
use std::time::Duration;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::UdpSocket;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_multiple_udp_nodes() {
    tokio::spawn(async {
        server().await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;

    let server = "127.0.0.1:8080";
    let mut handles = Vec::new();
    for seq in 1..4u64 {
        handles.push(tokio::spawn(async move {
            let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();

            let mut packet = Vec::new();
            let payload = format!("Node {} mesh-core", seq);
            packet.extend_from_slice(&seq.to_be_bytes());
            packet.extend_from_slice(payload.as_bytes());

            socket.send_to(&packet, server).await.unwrap();

            let mut buf = [0u8; 64];
            let (len, _address) =
                tokio::time::timeout(Duration::from_secs(5), socket.recv_from(&mut buf))
                    .await
                    .unwrap()
                    .unwrap();
            let ack = String::from_utf8_lossy(&buf[..len]).to_string();
            assert!(ack.contains(&seq.to_string()));
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
