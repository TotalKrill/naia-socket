use std::net::SocketAddr;

/// A Packet that can be sent to a Client
#[derive(Clone, Eq, PartialEq)]
pub struct Packet {
    pub address: SocketAddr,
    pub payload: Box<[u8]>,
}

impl Packet {
    pub fn new(address: SocketAddr, payload: Box<[u8]>) -> Self {
        Self { address, payload }
    }
}
