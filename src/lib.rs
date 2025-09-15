pub type MeshError = Box<dyn std::error::Error + Send + Sync>;
pub type StaticMeshError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod configure;
pub mod link;
pub mod types;
pub mod udpsocket;
pub mod utils;
pub mod wifi;
