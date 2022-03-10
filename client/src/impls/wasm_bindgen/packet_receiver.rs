use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::addr_cell::AddrCell;
use crate::{
    error::NaiaClientSocketError, packet::Packet, packet_receiver::PacketReceiverTrait,
    server_addr::ServerAddr,
};

/// Handles receiving messages from the Server through a given Client Socket
#[derive(Debug, Clone)]
pub struct PacketReceiverImpl {
    message_queue: Rc<RefCell<VecDeque<Packet>>>,
    server_addr: AddrCell,
}

impl PacketReceiverImpl {
    /// Create a new PacketReceiver, if supplied with the RtcDataChannel and a
    /// reference to a list of dropped messages
    pub fn new(message_queue: Rc<RefCell<VecDeque<Packet>>>, server_addr: AddrCell) -> Self {
        PacketReceiverImpl {
            message_queue,
            server_addr,
        }
    }
}

impl PacketReceiverTrait for PacketReceiverImpl {
    fn receive(&mut self) -> Result<Option<Packet>, NaiaClientSocketError> {
        match self.message_queue.borrow_mut().pop_front() {
            Some(packet) => {
                return Ok(Some(packet));
            }
            None => {
                return Ok(None);
            }
        }
    }

    /// Get the Server's Socket address
    fn server_addr(&self) -> ServerAddr {
        self.server_addr.get()
    }
}

unsafe impl Send for PacketReceiverImpl {}
unsafe impl Sync for PacketReceiverImpl {}
