use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use tokio::time;

use crate::types::peer::{PeerID, PeerInfo, PeerStore};

// sends message to the peers
pub async fn broadcast(sender_id: PeerID, sender_addr: SocketAddr) {
    // 0.0.0.0 binds all local addresses
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let message = format!("{}|{}", sender_id.0, sender_addr);

    tokio::spawn(async move {
        loop {
            match socket
                .send_to(message.as_bytes(), &format!("239.255.0.1:4000"))
                .await
            {
                Ok(_) => log::info!("Message sent to the node"),
                Err(e) => log::warn!("Failed to send UDP packet: {}", e),
            }

            let _ = time::sleep(Duration::from_secs(2));
        }
    });
}

// listens for messages in the network in multiple available interfaces and adds to the
// corresponding peer store
pub async fn listener(peer_store: Arc<Mutex<PeerStore>>, address: String) {
    // socket2 le socket modify garna help garcha
    // socket2 makes aeuta blockcing socket which cannot be converted to tokio socket
    // so non blocking is set
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
    socket.set_reuse_port(true).unwrap();
    let addr = address.parse::<SocketAddr>().unwrap();
    socket.bind(&SockAddr::from(addr)).unwrap();
    socket.set_nonblocking(true).unwrap();
    let std_socket: std::net::UdpSocket = socket.into();
    let tokio_socket: tokio::net::UdpSocket = tokio::net::UdpSocket::from_std(std_socket).unwrap();

    tokio_socket
        .join_multicast_v4(
            Ipv4Addr::new(239, 255, 0, 1),
            Ipv4Addr::new(0, 0, 0, 0).into(),
        )
        .unwrap();

    let mut buf = vec![0u8; 4096];

    loop {
        match tokio_socket.recv_from(&mut buf).await {
            Ok((len, _src)) => {
                let payload = String::from_utf8_lossy(&buf[..len]);
                match payload.split_once("|") {
                    Some((id, addr_str)) => match addr_str.parse::<SocketAddr>() {
                        Ok(addr) => {
                            let peer_info = PeerInfo {
                                id: PeerID(id.to_string()),
                                wifi_addr: Some(addr),
                                ble_addr: None,
                                last_seen: Instant::now(),
                                rtt_ms: None,
                                mtu: None,
                                loss_percent: None,
                            };
                            let mut peer_store = peer_store.lock().unwrap();
                            peer_store.update_store(peer_info);
                            log::info!("Discovered peer: {} at {}", id, addr);
                        }
                        Err(e) => log::warn!("Invalid peer address '{}': {}", addr_str, e),
                    },
                    None => log::warn!("Invalid payload format: {}", payload),
                }
            }
            Err(e) => {
                log::error!("Failed to receive UDP packet: {}", e);
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        }
    }
}
