use std::{
    cmp::min,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use anyhow::Result;
use bytes::Bytes;
use tokio::{net::UdpSocket, sync::mpsc};

use game_controller_msgs::{MONITOR_REQUEST_PORT, MONITOR_REQUEST_SIZE};

use crate::Event;

/// This struct represents a receiver for monitor requests. It listens only on the given local
/// address. Received messages are passed to the caller as events in a [tokio::sync::mpsc] channel.
pub struct MonitorRequestReceiver {
    socket: UdpSocket,
    event_sender: mpsc::UnboundedSender<Event>,
}

impl MonitorRequestReceiver {
    /// This function creates a new receiver for monitor requests.
    pub async fn new(address: IpAddr, event_sender: mpsc::UnboundedSender<Event>) -> Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind((
                match address {
                    IpAddr::V4(_) => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                    IpAddr::V6(_) => IpAddr::V6(Ipv6Addr::UNSPECIFIED),
                },
                MONITOR_REQUEST_PORT,
            ))
            .await?,
            event_sender,
        })
    }

    /// This function runs the receiver until an error occurs.
    pub async fn run(&self) -> Result<()> {
        let mut buffer = vec![0u8; MONITOR_REQUEST_SIZE + 1];
        loop {
            let (length, address) = crate::workaround::recv_from(&self.socket, &mut buffer).await?;
            self.event_sender.send(Event::MonitorRequest {
                host: address.ip(),
                data: Bytes::copy_from_slice(&buffer[0..min(length, MONITOR_REQUEST_SIZE)]),
                too_long: length > MONITOR_REQUEST_SIZE,
            })?;
        }
    }
}
