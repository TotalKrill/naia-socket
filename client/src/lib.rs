//! # Naia Client Socket
//! A Socket abstraction over either a UDP socket on native Linux, or a
//! unreliable WebRTC datachannel on the browser

#![deny(
    missing_docs,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(all(target_arch = "wasm32", feature = "wbindgen"))] {
        #[macro_use]
        extern crate serde_derive;
    }
    else {
    }
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod wasm_utils;
    }
    else {
        mod wasm_utils; // Remove this!
    }
}

mod error;
mod impls;
mod packet;
mod packet_receiver;
mod server_addr;

pub use naia_socket_shared::Timer;

pub use error::NaiaClientSocketError;
pub use impls::{PacketSender, Socket};
pub use packet::Packet;
pub use packet_receiver::PacketReceiver;
pub use server_addr::ServerAddr;

cfg_if! {
    if #[cfg(all(target_arch = "wasm32", feature = "wbindgen", feature = "mquad"))]
    {
        // Use both protocols...
        compile_error!("Naia Client Socket on Wasm requires either the 'wbindgen' OR 'mquad' feature to be enabled, you must pick one.");
    }
    else if #[cfg(all(target_arch = "wasm32", not(feature = "wbindgen"), not(feature = "mquad")))]
    {
        // Use no protocols...
        compile_error!("Naia Client Socket on Wasm requires either the 'wbindgen' or 'mquad' feature to be enabled, you must pick one.");
    }
}
