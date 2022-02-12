use crossbeam::channel::Receiver;

use naia_socket_shared::{link_condition_logic, LinkConditionerConfig, TimeQueue};

use super::{error::NaiaServerSocketError, packet::Packet};

/// Used to receive packets from the Server Socket
#[derive(Clone)]
pub struct PacketReceiver {
    inner: Box<dyn PacketReceiverTrait>,
}

impl PacketReceiver {
    /// Create a new PacketReceiver
    pub fn new(inner: Box<dyn PacketReceiverTrait>) -> Self {
        PacketReceiver { inner }
    }

    /// Receives a packet from the Server Socket
    pub fn receive(&mut self) -> Result<Option<Packet>, NaiaServerSocketError> {
        return self.inner.receive();
    }
}

/// Used to receive packets from the Server Socket
pub trait PacketReceiverTrait: PacketReceiverClone + Send + Sync {
    /// Receives a packet from the Server Socket
    fn receive(&mut self) -> Result<Option<Packet>, NaiaServerSocketError>;
}

/// Used to receive packets from the Server Socket
#[derive(Clone)]
pub struct PacketReceiverImpl {
    channel_receiver: Receiver<Result<Packet, NaiaServerSocketError>>,
}

impl PacketReceiverImpl {
    /// Creates a new PacketReceiver
    pub fn new(channel_receiver: Receiver<Result<Packet, NaiaServerSocketError>>) -> Self {
        PacketReceiverImpl { channel_receiver }
    }
}

impl PacketReceiverTrait for PacketReceiverImpl {
    fn receive(&mut self) -> Result<Option<Packet>, NaiaServerSocketError> {
        match self.channel_receiver.try_recv() {
            Ok(result) => match result {
                Ok(packet) => return Ok(Some(packet)),
                Err(_) => return Ok(None),
            },
            Err(_) => {
                return Ok(None);
            }
        }
    }
}

/// Used to receive packets from the Server Socket
#[derive(Clone)]
pub struct ConditionedPacketReceiverImpl {
    channel_receiver: Receiver<Result<Packet, NaiaServerSocketError>>,
    link_conditioner_config: LinkConditionerConfig,
    time_queue: TimeQueue<Packet>,
}

impl ConditionedPacketReceiverImpl {
    /// Creates a new PacketReceiver
    pub fn new(
        channel_receiver: Receiver<Result<Packet, NaiaServerSocketError>>,
        link_conditioner_config: &LinkConditionerConfig,
    ) -> Self {
        ConditionedPacketReceiverImpl {
            channel_receiver,
            link_conditioner_config: link_conditioner_config.clone(),
            time_queue: TimeQueue::new(),
        }
    }

    fn process_packet(&mut self, packet: Packet) {
        link_condition_logic::process_packet(
            &self.link_conditioner_config,
            &mut self.time_queue,
            packet,
        );
    }

    fn has_packet(&self) -> bool {
        self.time_queue.has_item()
    }

    fn receive(&mut self) -> Packet {
        self.time_queue.pop_item().unwrap()
    }
}

impl PacketReceiverTrait for ConditionedPacketReceiverImpl {
    fn receive(&mut self) -> Result<Option<Packet>, NaiaServerSocketError> {
        loop {
            match self.channel_receiver.try_recv() {
                Ok(result) => match result {
                    Err(_) => {
                        break; //TODO: Handle error here
                    }
                    Ok(packet) => {
                        self.process_packet(packet);
                    }
                },
                Err(_) => {
                    break; //TODO: Handle error here
                }
            }
        }

        if self.has_packet() {
            return Ok(Some(self.receive()));
        } else {
            return Ok(None);
        }
    }
}

/// Used to clone Box<dyn PacketReceiverTrait>
pub trait PacketReceiverClone {
    /// Clone the boxed PacketReceiver
    fn clone_box(&self) -> Box<dyn PacketReceiverTrait>;
}

impl<T: 'static + PacketReceiverTrait + Clone> PacketReceiverClone for T {
    fn clone_box(&self) -> Box<dyn PacketReceiverTrait> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn PacketReceiverTrait> {
    fn clone(&self) -> Box<dyn PacketReceiverTrait> {
        PacketReceiverClone::clone_box(self.as_ref())
    }
}
