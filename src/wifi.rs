use crate::{
    configure::make_endpoint,
    link::{Link, LinkConnection},
    types::wifi_quic::WifiQuicLinkConnection,
    MeshError,
};
use quinn::{rustls::pki_types::CertificateDer, Endpoint};
use rcgen::{Issuer, KeyPair};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct WifiQuicLink {
    pub endpoint: Endpoint,
}

impl WifiQuicLink {
    pub fn new(
        addr: &str,
        trusted_peers: &[CertificateDer<'static>],
        node_name: &str,
        issuer: &Issuer<'static, KeyPair>,
    ) -> Result<Self, MeshError> {
        let endpoint = make_endpoint(
            addr.parse::<SocketAddr>()?,
            trusted_peers,
            node_name,
            issuer,
        )?;
        Ok(Self { endpoint })
    }
}

#[async_trait::async_trait]
impl Link for WifiQuicLink {
    async fn dial(
        &self,
        address: &str,
    ) -> Result<Box<dyn LinkConnection + Send + Sync>, Box<dyn std::error::Error + Send + Sync>>
    {
        let connection = self
            .endpoint
            .connect(address.parse::<SocketAddr>()?, "localhost")?
            .await?;

        Ok(Box::new(WifiQuicLinkConnection {
            connection: Arc::new(Mutex::new(connection)),
        }))
    }

    async fn accept(
        &self,
    ) -> Result<Box<dyn LinkConnection + Send + Sync>, Box<dyn std::error::Error + Send + Sync>>
    {
        // Wait for an incoming handshake

        while let Some(connecting) = self.endpoint.accept().await {
            // Finish QUIC handshake
            match connecting.await {
                Ok(connection) => {
                    log::info!(
                        "Connection established to remote peer {}",
                        connection.remote_address()
                    );
                    return Ok(Box::new(WifiQuicLinkConnection {
                        connection: Arc::new(Mutex::new(connection)),
                    }));
                }
                Err(e) => {
                    log::error!("Error establishing connection: {}", e);
                    return Err(Box::new(e));
                }
            };
        }

        log::error!("Endpoint closed; no more incoming connections");
        Err("endpoint closed; no more incoming connections".into())
    }

    fn mtu(&self) -> usize {
        100
    }

    fn latency(&self) -> std::time::Duration {
        Duration::from_millis(10)
    }
}

#[async_trait::async_trait]
impl LinkConnection for WifiQuicLinkConnection {
    async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let connection = self.connection.lock().await;
        let (mut send, _receive) = connection.clone().open_bi().await?;
        send.write_all(data).await?;
        send.finish()?;
        log::info!("Data is successfully sent!");
        Ok(())
    }

    async fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let connection = self.connection.lock().await;
        let bi_stream = connection.accept_bi().await?;
        let (_send, mut receive) = bi_stream;

        let mut buf = vec![0u8; 1024];
        match receive.read(&mut buf).await {
            Ok(_) => {
                log::info!("Data read successfully!");
                Ok(buf)
            }
            Err(e) => {
                log::error!("Error occurred while reading data!");
                Err(e.into())
            }
        }
    }
}
