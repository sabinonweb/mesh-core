use super::peer::{PeerID, PeerStore};
use crate::{link::Link, MeshError};
use std::sync::{Arc, Mutex};

pub struct RoutingLayer<L: Link + Clone + Send + Sync + 'static> {
    pub link: L,
    pub peer_store: Arc<Mutex<PeerStore>>,
}

impl<L: Link + Clone + Send + Sync + 'static> RoutingLayer<L> {
    pub fn new(link: L, peer_store: Arc<Mutex<PeerStore>>) -> Self {
        Self { link, peer_store }
    }

    pub async fn send(&self, peer_id: PeerID, data: &[u8]) -> Result<(), MeshError> {
        let store = self.peer_store.lock().unwrap();
        if let Some(peer_info) = store.get_peer(peer_id.clone()) {
            if let Some(wifi_addr) = peer_info.wifi_addr {
                let addr = wifi_addr.to_string();
                let connection = self.link.dial(&addr).await.unwrap();
                connection.send(data).await.unwrap();
                log::info!("Sent message to {}", peer_id.0);
            }
        }
        Ok(())
    }
}
