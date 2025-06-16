use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use anyhow::Result;
use bytes::{Bytes, BytesMut};
use tokio::{net::UdpSocket, sync::broadcast};

use game_controller_msgs::STATUS_MESSAGE_FORWARD_PORT;

/// This struct represents a sender that forwards status messages to a monitor application. Each
/// message is prefixed with the IP address of its original sender. Messages arrive in the
/// unassembled form via a [tokio::sync::broadcast] channel.
pub struct StatusMessageForwarder {
    socket: UdpSocket,
    message_receiver: broadcast::Receiver<(IpAddr, Bytes)>,
}

impl StatusMessageForwarder {
    /// This function creates a new sender that forwards status messages to a monitor application.
    pub async fn new(
        address: IpAddr,
        message_receiver: broadcast::Receiver<(IpAddr, Bytes)>,
    ) -> Result<Self> {
        Ok(Self {
            socket: {
                let socket = UdpSocket::bind((
                    match address {
                        IpAddr::V4(_) => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                        IpAddr::V6(_) => IpAddr::V6(Ipv6Addr::UNSPECIFIED),
                    },
                    0u16,
                ))
                .await?;
                socket
                    .connect((address, STATUS_MESSAGE_FORWARD_PORT))
                    .await?;
                socket
            },
            message_receiver,
        })
    }

    /// This function runs the forwarder until an error occurs. Network errors during sending are
    /// ignored because the network might change and we shouldn't crash in that case.
    pub async fn run(&mut self) -> Result<()> {
        loop {
            let (source, buffer) = self.message_receiver.recv().await?;
            let prefixed_buffer = match source {
                IpAddr::V4(ip) => {
                    let octets = ip.octets();
                    assert!(octets.len() == 4);
                    let mut prefixed_buffer = BytesMut::new();
                    prefixed_buffer.extend_from_slice(&octets);
                    prefixed_buffer.extend(buffer);
                    prefixed_buffer.freeze()
                }
                _ => todo!("implement forwarding of IPv6 status messages"),
            };
            let _ = self.socket.send(&prefixed_buffer).await;
        }
    }
}
