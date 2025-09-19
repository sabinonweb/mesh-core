use quinn::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct WifiQuicLinkConnection {
    pub connection: Arc<Mutex<Connection>>,
}
