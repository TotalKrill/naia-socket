use std::net::SocketAddr;

use super::shared::{naia_create_u8_array, naia_send};
use crate::Packet;

/// Handles sending messages to the Server for a given Client Socket
#[derive(Clone)]
pub struct PacketSender {
    remote_addr: SocketAddr,
}

impl PacketSender {
    /// Create a new PacketSender, if supplied with the RtcDataChannel and a
    /// reference to a list of dropped messages
    pub fn new(remote_addr: SocketAddr) -> Self {
        PacketSender { remote_addr }
    }

    /// Send a Packet to the Server
    pub fn send(&mut self, packet: Packet) {
        unsafe {
            let payload: &[u8] = packet.payload();
            let ptr = payload.as_ptr();
            let len = payload.len();
            let js_obj = naia_create_u8_array(ptr as _, len as _);
            naia_send(js_obj);
        }
    }

    /// Get SocketAddr PacketSender is sending to
    pub fn remote_addr(&self) -> SocketAddr {
        self.remote_addr
    }
}
