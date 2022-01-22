use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{error::NaiaClientSocketError, packet::Packet, packet_receiver::PacketReceiverTrait};

/// Handles receiving messages from the Server through a given Client Socket
#[derive(Clone)]
pub struct PacketReceiverImpl {
    message_queue: Rc<RefCell<VecDeque<Packet>>>,
}

impl PacketReceiverImpl {
    /// Create a new PacketReceiver, if supplied with the RtcDataChannel and a
    /// reference to a list of dropped messages
    pub fn new(message_queue: Rc<RefCell<VecDeque<Packet>>>) -> Self {
        PacketReceiverImpl { message_queue }
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
}

unsafe impl Send for PacketReceiverImpl {}
unsafe impl Sync for PacketReceiverImpl {}
