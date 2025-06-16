use std::{cmp::min, net::IpAddr};

use anyhow::Result;
use bytes::Bytes;
use tokio::{net::UdpSocket, sync::mpsc};

use game_controller_msgs::{STATUS_MESSAGE_PORT, STATUS_MESSAGE_SIZE};

use crate::Event;

/// This struct represents a receiver for status messages. Status messages are UDP packets with a
/// fixed format, although that format isn't checked here. It listens only on the given local
/// address. Received messages are passed to the caller as events in a [tokio::sync::mpsc] channel.
pub struct StatusMessageReceiver {
    socket: UdpSocket,
    event_sender: mpsc::UnboundedSender<Event>,
}

impl StatusMessageReceiver {
    /// This function creates a new receiver for status messages.
    pub async fn new(address: IpAddr, event_sender: mpsc::UnboundedSender<Event>) -> Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind((address, STATUS_MESSAGE_PORT)).await?,
            event_sender,
        })
    }

    /// This function runs the receiver until an error occurs.
    pub async fn run(&self) -> Result<()> {
        let mut buffer = vec![0u8; STATUS_MESSAGE_SIZE + 1];
        loop {
            let (length, address) = crate::workaround::recv_from(&self.socket, &mut buffer).await?;
            self.event_sender.send(Event::StatusMessage {
                host: address.ip(),
                data: Bytes::copy_from_slice(&buffer[0..min(length, STATUS_MESSAGE_SIZE)]),
                too_long: length > STATUS_MESSAGE_SIZE,
            })?;
        }
    }
}
