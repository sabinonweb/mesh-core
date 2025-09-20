use std::{collections::HashMap, net::SocketAddr, time::Instant};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LinkType {
    Wifi,
    Ble,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PeerID(pub String);

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub id: PeerID,
    pub wifi_addr: Option<SocketAddr>,
    pub ble_addr: Option<String>,
    pub last_seen: Instant,
    pub rtt_ms: Option<u32>,
    pub mtu: Option<usize>,
    pub loss_percent: Option<f32>,
}

#[derive(Default, Debug)]
pub struct PeerStore {
    pub peers: HashMap<PeerID, PeerInfo>,
}

impl PeerStore {
    pub fn update_store(&mut self, info: PeerInfo) {
        self.peers.insert(info.clone().id, info);
    }

    pub fn get_peer(&self, id: PeerID) -> Option<&PeerInfo> {
        self.peers.get(&id)
    }

    pub fn get_all_peers(&self) -> Vec<PeerInfo> {
        self.peers.values().cloned().collect()
    }
}
