use quinn::rustls::pki_types::PrivatePkcs8KeyDer;
use quinn::{rustls, Endpoint};
use quinn::{rustls::pki_types::CertificateDer, ClientConfig, ServerConfig};
use std::collections::HashSet;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

pub fn handle_packet(seen: &mut HashSet<u64>, seq: u64, payload: &[u8]) -> bool {
    if seen.contains(&seq) {
        log::info!("Duplicate packet {}", seq);
        false
    } else {
        seen.insert(seq);
        println!("Hashset {:?}", seen);
        log::info!("Received {}: {}", seq, String::from_utf8_lossy(payload));
        true
    }
}
