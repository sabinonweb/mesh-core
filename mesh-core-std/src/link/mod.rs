pub mod discovery;
pub mod link_trait;
pub mod multilink;

pub trait Link {
    fn send(&self, data: Vec<u8>);
    fn recv(&self) -> Option<Vec<u8>>;
}
