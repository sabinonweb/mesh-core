use crate::{
    configure::make_endpoint,
    link::{Link, LinkConnection},
    MeshError,
};
use quinn::{Connection, Endpoint};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::Mutex;

pub struct WifiQuicLink {
    pub endpoint: Endpoint,
}

impl WifiQuicLink {
    pub fn new(addr: &str) -> Result<Self, MeshError> {
        let endpoint = make_endpoint(addr.parse::<SocketAddr>()?)?;
        Ok(Self { endpoint })
    }
}

#[async_trait::async_trait]
impl Link for WifiQuicLink {
    async fn dial(
        &self,
        address: &str,
    ) -> Result<Box<dyn LinkConnection>, Box<dyn std::error::Error + Send + Sync>> {
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
        if let Some(connecting) = self.endpoint.accept().await {
            // Finish QUIC handshake
            match connecting.await {
                Ok(connection) => {
                    log::info!(
                        "Connection established to remote peer {}",
                        connection.remote_address()
                    );
                    Ok(Box::new(WifiQuicLinkConnection {
                        connection: Arc::new(Mutex::new(connection)),
                    }))
                }
                Err(e) => {
                    log::error!("Error establishing connection: {}", e);
                    Err(Box::new(e)) // return the actual error, not a string
                }
            }
        } else {
            log::error!("No incoming connection");
            Err("No incoming connection".into())
        }
    }
    fn mtu(&self) -> usize {
        100
    }

    fn latency(&self) -> std::time::Duration {
        Duration::from_millis(10)
    }
}

pub struct WifiQuicLinkConnection {
    connection: Arc<Mutex<Connection>>,
}

#[async_trait::async_trait]
impl LinkConnection for WifiQuicLinkConnection {
    async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let connection = self.connection.lock().await;
        let (mut send, _receive) = connection.clone().open_bi().await?;
        match send.write_all(data).await {
            Ok(()) => {
                log::info!("Data is successfully sent!");
                Ok(())
            }
            Err(e) => {
                log::error!("Error occurred while sending data!");
                Err(e.into())
            }
        }
    }

    async fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let connection = self.connection.lock().await;
        let bi_stream = connection.accept_bi().await?;
        let (_send, mut receive) = bi_stream;

        let mut buf = Vec::new();
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
