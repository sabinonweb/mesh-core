use crate::{configure::make_endpoint, MeshError};
use quinn::{rustls::pki_types::CertificateDer, Connection, Endpoint};
use rcgen::{Issuer, KeyPair};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct WifiQuicLinkConnection {
    pub connection: Arc<Mutex<Connection>>,
}
