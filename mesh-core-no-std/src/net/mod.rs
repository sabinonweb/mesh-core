pub mod ble;
pub mod wifi;

pub trait Link {
    fn send(&self, data: &[u8]);
    fn recv(&self) -> Option<Vec<u8>>;
}
