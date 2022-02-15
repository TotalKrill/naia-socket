use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use web_sys::RtcDataChannel;

use super::addr_cell::AddrCell;
use crate::{server_addr::ServerAddr, Packet};

/// Handles sending messages to the Server for a given Client Socket
#[derive(Clone)]
pub struct PacketSender {
    data_channel: RtcDataChannel,
    dropped_outgoing_messages: Rc<RefCell<VecDeque<Packet>>>,
    server_addr: AddrCell,
}

impl PacketSender {
    /// Create a new PacketSender, if supplied with the RtcDataChannel and a
    /// reference to a list of dropped messages
    pub fn new(
        data_channel: RtcDataChannel,
        dropped_outgoing_messages: Rc<RefCell<VecDeque<Packet>>>,
        server_addr: AddrCell,
    ) -> Self {
        PacketSender {
            data_channel,
            dropped_outgoing_messages,
            server_addr,
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

    /// Get the Server's Socket address
    pub fn server_addr(&self) -> ServerAddr {
        self.server_addr.get()
    }
}

unsafe impl Send for PacketSender {}
unsafe impl Sync for PacketSender {}
