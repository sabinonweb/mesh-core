pub type MeshError = Box<dyn std::error::Error + Send + Sync>;
pub type StaticMeshError = Box<dyn std::error::Error + Send + Sync + 'static>;

// generated code is stored in mesh.rs
pub mod mesh {
    include!(concat!(env!("OUT_DIR"), "/mesh.rs"));
}

pub mod ble;
pub mod configure;
pub mod connection;
pub mod discovery;
pub mod link;
pub mod types;
pub mod udpsocket;
pub mod utils;
pub mod wifi;
