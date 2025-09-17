use std::sync::{Arc, Mutex};

use mesh_core::{
    discovery::{broadcast, listener},
    types::peer::{PeerID, PeerStore},
};

#[tokio::test]
async fn discovery() {
    let peer_store = Arc::new(Mutex::new(PeerStore::default()));
    println!("PeerStore: {:?}", peer_store);
    let peer_id = PeerID("peer1".to_string());
    let peer_addr = "127.0.0.1:12345".parse().unwrap();
    let address = "0.0.0.0:5000".to_string();

    tokio::spawn(async {
        listener(peer_store, address).await;
    });

    broadcast(peer_id, peer_addr).await;
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
}
