use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use web_sys::RtcDataChannel;

use crate::Packet;

/// Handles sending messages to the Server for a given Client Socket
#[derive(Clone)]
pub struct PacketSender {
    data_channel: RtcDataChannel,
    dropped_outgoing_messages: Rc<RefCell<VecDeque<Packet>>>,
}

impl PacketSender {
    /// Create a new PacketSender, if supplied with the RtcDataChannel and a
    /// reference to a list of dropped messages
    pub fn new(
        data_channel: RtcDataChannel,
        dropped_outgoing_messages: Rc<RefCell<VecDeque<Packet>>>,
    ) -> Self {
        PacketSender {
            data_channel,
            dropped_outgoing_messages,
        }
    }

    /// Send a Packet to the Server
    pub fn send(&mut self, packet: Packet) {
        self.resend_dropped_messages();

        if let Err(err) = self.data_channel.send_with_u8_array(&packet.payload()) {
            log::info!("error when sending packet: {:?}", err);

            self.dropped_outgoing_messages
                .borrow_mut()
                .push_back(packet);
        }
    }

    fn resend_dropped_messages(&mut self) {
        if !self.dropped_outgoing_messages.borrow().is_empty() {
            if let Some(dropped_packets) = {
                let mut dom = self.dropped_outgoing_messages.borrow_mut();
                let dropped_packets: Vec<Packet> = dom.drain(..).collect::<Vec<Packet>>();
                Some(dropped_packets)
            } {
                for dropped_packet in dropped_packets {
                    self.send(dropped_packet);
                }
            }
        }
    }
}

unsafe impl Send for PacketSender {}
unsafe impl Sync for PacketSender {}
