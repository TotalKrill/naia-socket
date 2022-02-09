extern crate log;

use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

use url::Url;

use naia_socket_shared::{find_my_ip_address, SocketConfig};

use crate::{
    packet_receiver::{ConditionedPacketReceiver, PacketReceiver, PacketReceiverTrait},
    url_parse::get_url,
};

use super::{packet_receiver::PacketReceiverImpl, packet_sender::PacketSender};

/// A client-side socket which communicates with an underlying unordered &
/// unreliable protocol
pub struct Socket {
    config: SocketConfig,
    io: Option<Io>,
}

/// Contains internal socket packet sender/receiver
struct Io {
    /// Used to send packets through the socket
    pub packet_sender: PacketSender,
    /// Used to receive packets from the socket
    pub packet_receiver: PacketReceiver,
}

impl Socket {
    /// Create a new Socket
    pub fn new(config: SocketConfig) -> Self {
        Socket { config, io: None }
    }

    /// Connects to the given server address
    pub fn connect(&mut self, server_session_url: &str) {
        if self.io.is_some() {
            panic!("Socket already listening!");
        }

        let server_url = get_url(server_session_url);
        let server_socket_addr = get_socket_addr(&server_url);

        let client_ip_address =
            find_my_ip_address().expect("cannot find host's current IP address");

        let socket = Arc::new(Mutex::new(UdpSocket::bind((client_ip_address, 0)).unwrap()));
        socket
            .as_ref()
            .lock()
            .unwrap()
            .set_nonblocking(true)
            .expect("can't set socket to non-blocking!");

        let packet_sender = PacketSender::new(server_socket_addr, socket.clone());

        let conditioner_config = self.config.link_condition_config.clone();

        let sender = packet_sender.clone();
        let receiver: Box<dyn PacketReceiverTrait> = {
            let inner_receiver =
                Box::new(PacketReceiverImpl::new(server_socket_addr, socket.clone()));
            if let Some(config) = &conditioner_config {
                Box::new(ConditionedPacketReceiver::new(inner_receiver, config))
            } else {
                inner_receiver
            }
        };

        self.io = Some(Io {
            packet_sender: sender,
            packet_receiver: PacketReceiver::new(receiver),
        });
    }

    /// Gets a PacketSender which can be used to send packets through the Socket
    pub fn get_packet_sender(&self) -> PacketSender {
        return self
            .io
            .as_ref()
            .expect("Socket is not connected yet! Call Socket.connect() before this.")
            .packet_sender
            .clone();
    }

    /// Gets a PacketReceiver which can be used to receive packets from the
    /// Socket
    pub fn get_packet_receiver(&self) -> PacketReceiver {
        return self
            .io
            .as_ref()
            .expect("Socket is not connected yet! Call Socket.connect() before this.")
            .packet_receiver
            .clone();
    }
}

fn get_socket_addr(url: &Url) -> SocketAddr {
    const SOCKET_PARSE_FAIL_STR: &str = "could not get SocketAddr from input URL";

    let addr_list = url
        .socket_addrs(|| match url.scheme() {
            "http" => Some(80),
            "https" => Some(443),
            _ => None,
        })
        .expect(SOCKET_PARSE_FAIL_STR);

    if addr_list.is_empty() {
        panic!("{}", SOCKET_PARSE_FAIL_STR);
    }

    return addr_list.first().expect(SOCKET_PARSE_FAIL_STR).clone();
}
