use std::net::IpAddr;

use anyhow::Result;
use bytes::Bytes;
use tokio::{net::UdpSocket, sync::mpsc};

use crate::Event;

/// This struct represents a receiver for status messages. Status messages are UDP packets with a
/// fixed format, although that format isn't checked here. It listens only on the given local
/// address. Received messages are passed to the caller as events in a [tokio::sync::mpsc] channel.
pub struct RefereeReceiver {
    socket: UdpSocket,
    event_sender: mpsc::UnboundedSender<Event>,
}

impl RefereeReceiver {
    /// This function creates a new receiver for status messages.
    pub async fn new(address: IpAddr, event_sender: mpsc::UnboundedSender<Event>) -> Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind((address, 4040)).await?,
            event_sender,
        })
    }

    /// This function runs the receiver until an error occurs.
    pub async fn run(&self) -> Result<()> {
        let mut buffer = vec![0u8; 10 + 1];
        loop {
            let (_length, _address) =
                crate::workaround::recv_from(&self.socket, &mut buffer).await?;
            println!("DEBUG1 {:?}", Bytes::copy_from_slice(&buffer[..]));
            self.event_sender.send(Event::RefereeMessage {
                data: Bytes::copy_from_slice(&buffer[..]),
            })?;
        }
    }
}
