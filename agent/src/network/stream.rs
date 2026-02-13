use crossbeam::channel;
use shared::{error::NetworkError, multiplexing::StreamId, packets::Packet};
use std::sync::Arc;

use super::multiplex::MultiplexManager;

pub struct Stream {
    pub id: StreamId,
    pub manager: Arc<MultiplexManager>,
    pub rx: channel::Receiver<Vec<u8>>,
}

impl Stream {
    pub(crate) fn new(
        id: StreamId,
        manager: Arc<MultiplexManager>,
        rx: channel::Receiver<Vec<u8>>,
    ) -> Self {
        Self { id, manager, rx }
    }

    pub fn id(&self) -> StreamId {
        self.id
    }

    #[allow(dead_code)]
    pub fn send<P: Packet>(&self, packet: &P) -> Result<(), NetworkError> {
        let data = packet.serialize()?;
        self.send_bytes(&data)
    }

    pub fn send_bytes(&self, data: &[u8]) -> Result<(), NetworkError> {
        self.manager.send_on_stream(self.id, data.to_vec())
    }

    pub fn receive(&self) -> Result<Vec<u8>, NetworkError> {
        self.rx
            .recv()
            .map_err(|_| NetworkError::ChannelReceiveError)
    }

    #[allow(dead_code)]
    pub fn close(self) -> Result<(), NetworkError> {
        // The stream will be automatically closed when dropped
        // The MultiplexManager will detect this and clean up
        self.manager.close_stream(self.id)
    }
}
