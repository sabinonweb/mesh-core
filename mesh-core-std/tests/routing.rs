use mesh_core::{
    discovery::{broadcast, listener},
    types::{
        peer::{PeerID, PeerStore},
        routing::RoutingLayer,
    },
    utils::{generate_certificate_authority, generate_node_certs},
    wifi::WifiQuicLink,
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

#[tokio::test]
async fn routing() {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut nodes = vec![];
    let (ca_cert, ca_issuer) = generate_certificate_authority();
    let (_node1_cert, _node1_key) = generate_node_certs(&ca_issuer, "Node 1");
    let (_node2_cert, _node2_key) = generate_node_certs(&ca_issuer, "Node 2");

    for (i, _port) in [5000, 5001, 5002].iter().enumerate() {
        let id = PeerID(format!("node{}", i + 1));
        let addr = format!("127.0.0.1:4000");
        let address: SocketAddr = addr.clone().parse().unwrap();
        let peer_store = Arc::new(Mutex::new(PeerStore::default()));

        tokio::spawn(listener(peer_store.clone(), "0.0.0.0:4000".to_string()));

        tokio::spawn(broadcast(id.clone(), address));

        let link = match WifiQuicLink::new(
            &addr,
            &[ca_cert.der().clone().into_owned()],
            &id.0,
            &ca_issuer,
        ) {
            Ok(link) => {
                log::info!("WifiQuicLink formed for {}", addr);
                link
            }
            Err(e) => {
                log::error!("Error occurred while forming WifiQuicLink: {e}");
                return;
            }
        };

        let routing = RoutingLayer::new(link.clone(), peer_store.clone());

        nodes.push((id, peer_store.clone(), routing));
    }

    let _ = tokio::time::sleep(Duration::from_secs(20)).await;

    for (id, _, routing) in &nodes {
        for (target, _, _) in &nodes {
            if id != target {
                let result = routing
                    .send(
                        target.clone(),
                        &format!("Hello from {} to {:?}", id.0, target.0).as_bytes(),
                    )
                    .await;
                assert!(
                    result.is_ok(),
                    "Sending packet from {} to {} failed",
                    id.0,
                    target.0
                );
            }
        }
    }

    for (id, peer_store, _) in &nodes {
        let s = peer_store.lock().unwrap();
        println!("Node {} knows {} peers", id.0, s.get_all_peers().len());
    }
}
