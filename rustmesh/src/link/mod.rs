pub trait Link {
    fn send(&self, data: Vec<u8>);
    fn recv(&self) -> Vec<u8>;
}
