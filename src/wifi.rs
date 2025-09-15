use std::net::SocketAddr;

use quinn::Endpoint;

use crate::{configure::make_client_endpoint, MeshError};

pub struct WifiQuicLink {
    pub endpoint: Endpoint,
}

impl WifiQuicLink {
    pub fn new(addr: &str, certificates: &[&[u8]]) -> Result<Self, MeshError> {
        let endpoint = make_client_endpoint(certificates, addr.parse::<SocketAddr>()?)?;
        Ok(Self { endpoint })
    }
}
