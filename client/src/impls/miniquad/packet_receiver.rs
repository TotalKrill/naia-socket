use crate::{
    error::NaiaClientSocketError, packet::Packet, packet_receiver::PacketReceiverTrait,
    server_addr::ServerAddr,
};

use super::shared::{naia_resend_dropped_messages, ERROR_QUEUE, MESSAGE_QUEUE, SERVER_ADDR};

/// Handles receiving messages from the Server through a given Client Socket
#[derive(Clone)]
pub struct PacketReceiverImpl;

impl PacketReceiverImpl {
    /// Create a new PacketReceiver, if supplied with the RtcDataChannel and a
    /// reference to a list of dropped messages
    pub fn new() -> Self {
        PacketReceiverImpl {}
    }
}

impl PacketReceiverTrait for PacketReceiverImpl {
    fn receive(&mut self) -> Result<Option<Packet>, NaiaClientSocketError> {
        unsafe {
            naia_resend_dropped_messages();

            if let Some(msg_queue) = &mut MESSAGE_QUEUE {
                if let Some(message) = msg_queue.pop_front() {
                    return Ok(Some(Packet::new_raw(message)));
                }
            }

            if let Some(error_queue) = &mut ERROR_QUEUE {
                if let Some(error) = error_queue.pop_front() {
                    return Err(NaiaClientSocketError::Message(error));
                }
            }
        };

        Ok(None)
    }

    /// Get the Server's Socket address
    fn server_addr(&self) -> ServerAddr {
        unsafe { SERVER_ADDR }
    }
}
