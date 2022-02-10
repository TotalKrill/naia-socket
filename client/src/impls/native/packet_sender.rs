use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

use crate::Packet;

/// Handles sending messages to the Server for a given Client Socket
#[derive(Clone)]
pub struct PacketSender {
    remote_addr: SocketAddr,
    local_addr: SocketAddr,
    local_socket: Arc<Mutex<UdpSocket>>,
}

impl PacketSender {
    /// Create a new PacketSender, if supplied with the Server's address & a
    /// reference back to the parent Socket
    pub fn new(remote_addr: SocketAddr, local_socket: Arc<Mutex<UdpSocket>>) -> Self {
        let local_addr = local_socket.as_ref().lock().unwrap().local_addr().unwrap();
        PacketSender {
            remote_addr,
            local_socket,
            local_addr,
        }
    }

    /// Send a Packet to the Server
    pub fn send(&mut self, packet: Packet) {
        //send it
        if let Err(_) = self
            .local_socket
            .as_ref()
            .lock()
            .unwrap()
            .send_to(&packet.payload(), self.remote_addr)
        {
            //TODO: handle this error
        }
    }

    /// Get SocketAddr PacketSender is sending to
    pub fn remote_addr(&self) -> SocketAddr {
        self.remote_addr
    }

    /// Get SocketAddr PacketSender is sending from
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
}
