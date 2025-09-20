use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait Link {
    // Initias a connection to remote address
    async fn dial(
        &self,
        address: &str,
    ) -> Result<Box<dyn LinkConnection + Send + Sync>, Box<dyn std::error::Error + Send + Sync>>;

    // Accepts an incoming connection
    async fn accept(
        &self,
    ) -> Result<Box<dyn LinkConnection + Send + Sync>, Box<dyn std::error::Error + Send + Sync>>;

    // Maximum packet size for this link
    fn mtu(&self) -> usize;

    // Estimated Latency
    fn latency(&self) -> Duration;
}

// Single active connection over a link
#[async_trait]
pub trait LinkConnection {
    async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>;
}
