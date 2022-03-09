use std::{
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

use crate::{
    error::NaiaClientSocketError, packet::Packet, packet_receiver::PacketReceiverTrait,
    server_addr::ServerAddr,
};

/// Handles receiving messages from the Server through a given Client Socket
#[derive(Clone)]
pub struct PacketReceiverImpl {
    /// The first response we get, is treated as the server, to block others
    /// attempt from responding
    server_addr: Option<SocketAddr>,
    local_socket: Arc<Mutex<UdpSocket>>,
    receive_buffer: Vec<u8>,
}

use std::fmt;
impl fmt::Debug for PacketReceiverImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PacketReceiverImpl")
            .field("server_address", &self.server_addr)
            .field("local_socket", &self.local_socket)
            .field("recieve_buffer", &format!("[omitted...]"))
            .finish()
    }
}

impl PacketReceiverImpl {
    /// Create a new PacketReceiver, if supplied with the Server's address & a
    /// reference back to the parent Socket
    pub fn new(local_socket: Arc<Mutex<UdpSocket>>) -> Self {
        PacketReceiverImpl {
            /// This is set upon first reception of message
            server_addr: None,
            local_socket,
            receive_buffer: vec![0; 1472],
        }
    }
}

impl PacketReceiverTrait for PacketReceiverImpl {
    fn receive(&mut self) -> Result<Option<Packet>, NaiaClientSocketError> {
        let buffer: &mut [u8] = self.receive_buffer.as_mut();
        match self
            .local_socket
            .as_ref()
            .lock()
            .unwrap()
            .recv_from(buffer)
            .map(move |(recv_len, address)| (&buffer[..recv_len], address))
        {
            Ok((payload, address)) => {
                if let None = self.server_addr {
                    self.server_addr = Some(address);
                }
                if let Some(server_addr) = self.server_addr {
                    if address == server_addr {
                        return Ok(Some(Packet::new(payload.to_vec())));
                    } else {
                        let err_message = format!(
                            "Received packet from unknown sender with a socket
                 address of: {}",
                            address
                        );
                        log::error!("{}", err_message);
                        return Err(NaiaClientSocketError::Message(err_message.to_string()));
                    }
                }

                unreachable!("Server address should have been set before");
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                //just didn't receive anything this time
                return Ok(None);
            }
            Err(e) => {
                return Err(NaiaClientSocketError::Wrapped(Box::new(e)));
            }
        }
    }

    /// Get the Server's Socket address
    fn server_addr(&self) -> ServerAddr {
        match self.server_addr {
            Some(server_addr) => ServerAddr::Found(server_addr),
            None => ServerAddr::Finding,
        }
    }
}
