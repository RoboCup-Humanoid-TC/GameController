use std::{
    cmp::min,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
};

use anyhow::Result;
use bytes::Bytes;
use socket2::{Protocol, SockAddr, Socket, Type};
use tokio::{net::UdpSocket, sync::mpsc};

use game_controller_msgs::{TEAM_MESSAGE_MAX_SIZE, TEAM_MESSAGE_PORT_BASE};

use crate::Event;

/// This struct represents a receiver for team messages. The messages are UDP packets of a given
/// maximum length. It listens on any local address, but by specifying a local address, the caller
/// can choose between IPv4 and IPv6. The given team determines the UDP port on which messages are
/// expected. Received messages are passed to the caller as events in a [tokio::sync::mpsc]
/// channel.
pub struct TeamMessageReceiver {
    socket: UdpSocket,
    team: u8,
    event_sender: mpsc::UnboundedSender<Event>,
}

impl TeamMessageReceiver {
    /// This function creates a new receiver for team messages.
    pub async fn new(
        address: IpAddr,
        multicast: bool,
        team: u8,
        event_sender: mpsc::UnboundedSender<Event>,
    ) -> Result<Self> {
        Ok(Self {
            socket: {
                // This might be stuff that should not be done in an async function.
                let socket_address: SockAddr = SocketAddr::new(
                    match address {
                        IpAddr::V4(_) => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                        IpAddr::V6(_) => IpAddr::V6(Ipv6Addr::UNSPECIFIED),
                    },
                    TEAM_MESSAGE_PORT_BASE + (team as u16),
                )
                .into();
                let raw_socket =
                    Socket::new(socket_address.domain(), Type::DGRAM, Some(Protocol::UDP))?;
                #[cfg(target_os = "macos")]
                raw_socket.set_reuse_port(true)?;
                #[cfg(any(target_os = "linux", target_os = "windows"))]
                raw_socket.set_reuse_address(true)?;
                // Extend this for other operating systems when it's clear what the right thing is
                // on that system.
                raw_socket.bind(&socket_address)?;
                raw_socket.set_nonblocking(true)?;
                let socket = UdpSocket::from_std(raw_socket.into())?;
                if multicast {
                    if let IpAddr::V4(address_v4) = address {
                        let _ = socket.join_multicast_v4(Ipv4Addr::new(239, 0, 0, 1), address_v4);
                    }
                }
                socket
            },
            team,
            event_sender,
        })
    }

    /// This function runs the receiver until an error occurs.
    pub async fn run(&self) -> Result<()> {
        // Since we want to catch team messages that are too long, we expect one more byte than the
        // maximum message length.
        let mut buffer = vec![0u8; TEAM_MESSAGE_MAX_SIZE + 1];
        loop {
            let (length, address) = crate::workaround::recv_from(&self.socket, &mut buffer).await?;
            self.event_sender.send(Event::TeamMessage {
                host: address.ip(),
                team: self.team,
                data: Bytes::copy_from_slice(&buffer[0..min(length, TEAM_MESSAGE_MAX_SIZE)]),
                too_long: length > TEAM_MESSAGE_MAX_SIZE,
            })?;
        }
    }
}
