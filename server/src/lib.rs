//! # Naia Server Socket
//! Provides an abstraction of a Socket capable of sending/receiving to many clients, using either
//! an underlying UdpSocket or a service that can communicate via unreliable WebRTC datachannels

#![deny(missing_docs,
    missing_debug_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces, unused_qualifications)]

extern crate log;

#[macro_use]
extern crate cfg_if;

#[cfg(feature = "use-udp")]
mod udp_server_socket;
#[cfg(feature = "use-webrtc")]
mod webrtc_server_socket;

mod error;
mod message_sender;
mod packet;
mod server_socket;
mod socket_event;

pub use error::NaiaServerSocketError;
pub use message_sender::MessageSender;
pub use naia_socket_shared::{find_my_ip_address, Config};
pub use packet::Packet;
pub use server_socket::ServerSocket;
pub use socket_event::SocketEvent;
