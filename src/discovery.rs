use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use tokio::{net::UdpSocket, time};

use crate::types::peer::{PeerID, PeerInfo, PeerStore};

pub async fn broadcast(peer_id: PeerID, peer_addr: SocketAddr) {
    // 0.0.0.0 binds all local addresses
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let message = format!("{}|{}", peer_id.0, peer_addr);

    tokio::spawn(async move {
        loop {
            match socket.send_to(message.as_bytes(), "239.255.0.1:4000").await {
                Ok(_) => log::info!("Message sent to the node"),
                Err(e) => log::warn!("Failed to send UDP packet: {}", e),
            }

            let _ = time::sleep(Duration::from_secs(100));
        }
    });
}

pub async fn listener(peer_store: Arc<Mutex<PeerStore>>) {
    let socket = UdpSocket::bind("0.0.0.0:4000").await.unwrap();
    socket
        .join_multicast_v4(
            Ipv4Addr::new(239, 255, 0, 1),
            Ipv4Addr::new(0, 0, 0, 0).into(),
        )
        .unwrap();

    let mut buf = vec![0u8; 4096];

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, _src)) => {
                let payload = String::from_utf8_lossy(&buf[..len]);
                match payload.split_once("|") {
                    Some((id, addr_str)) => match addr_str.parse::<SocketAddr>() {
                        Ok(addr) => {
                            let peer_info = PeerInfo {
                                id: PeerID(id.to_string()),
                                wifi_addr: Some(addr),
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
