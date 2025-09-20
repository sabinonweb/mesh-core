use crate::{
    types::peer::{LinkType, PeerID, PeerInfo, PeerStore},
    MeshError,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Instant};
use tokio::sync::Mutex;

use super::link_trait::Link;

pub struct MultiLinkManager {
    pub peer_store: Arc<Mutex<PeerStore>>,
    pub links: HashMap<LinkType, Box<dyn Link + Send + Sync>>,
    pub bootstraps: Vec<(PeerID, SocketAddr)>,
    pub priority: Vec<LinkType>,
}

impl MultiLinkManager {
    pub fn new(
        links: HashMap<LinkType, Box<dyn Link + Send + Sync>>,
        bootstraps: Vec<(PeerID, SocketAddr)>,
        priority: Vec<LinkType>,
    ) -> Self {
        Self {
            peer_store: Arc::new(Mutex::new(PeerStore::default())),
            links,
            bootstraps,
            priority,
        }
    }

    pub async fn bootstrap_peers(&self) {
        let mut store = self.peer_store.lock().await;
        for (id, addr) in &self.bootstraps {
            let peer = PeerInfo {
                id: id.clone(),
                wifi_addr: Some(*addr),
                ble_addr: None,
                last_seen: Instant::now(),
                rtt_ms: Some(0),
                mtu: Some(1500),
                loss_percent: Some(0.0),
            };
            store.update_store(peer);
        }
    }

    pub async fn pick_best_link(&self, peer_id: &PeerID) -> Option<LinkType> {
        let store = self.peer_store.lock().await;
        let peer = store.get_peer(peer_id.clone()).unwrap();
        for lt in &self.priority {
            match lt {
                LinkType::Wifi if peer.wifi_addr.is_some() => return Some(LinkType::Wifi),
                LinkType::Ble if peer.ble_addr.is_some() => return Some(LinkType::Ble),
                _ => {}
            }
        }
        None
    }

    pub async fn send(&self, peer_id: &PeerID, data: &[u8]) -> Result<(), MeshError> {
        let store = self.peer_store.lock().await;
        if let Some(peer) = store.get_peer(peer_id.clone()) {
            for lt in &self.priority {
                let send_result = match lt {
                    LinkType::Wifi => {
                        if let Some(addr) = peer.wifi_addr {
                            if let Some(link) = self.links.get(lt) {
                                match link.dial(&addr.to_string()).await {
                                    Ok(conn) => conn.send(data).await,
                                    Err(e) => {
                                        log::warn!("wi-fi dial failed for {}: {}", peer_id.0, e);
                                        continue;
                                    }
                                }
                            } else {
                                log::warn!("wi-fi link not available in manager");
                                continue;
                            }
                        } else {
                            log::warn!("peer {} has no wi-fi address", peer_id.0);
                            continue;
                        }
                    }
                    LinkType::Ble => {
                        if let Some(addr) = &peer.ble_addr {
                            if let Some(link) = self.links.get(lt) {
                                match link.dial(addr).await {
                                    Ok(conn) => conn.send(data).await,
                                    Err(e) => {
                                        log::warn!("ble dial failed for {}: {}", peer_id.0, e);
                                        continue;
                                    }
                                }
                            } else {
                                log::warn!("ble link not available in manager");
                                continue;
                            }
                        } else {
                            log::warn!("peer {} has no ble address", peer_id.0);
                            continue;
                        }
                    }
                };
                if let Ok(_) = send_result {
                    log::info!("Successfully sent data to {} via {:?}", peer_id.0, lt);
                    return Ok(());
                } else {
                    log::warn!("Failed to send data via {:?} to {}", lt, peer_id.0);
                }
            }
        }
        log::error!("No route available to peer {}", peer_id.0);
        Err("No route available via any link".into())
    }
}
