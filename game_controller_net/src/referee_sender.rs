use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    time::Duration,
};

use anyhow::Result;
use bytes::Bytes;
use tokio::{net::UdpSocket, sync::watch, time::interval};


use game_controller_core::types::{Game, Params};
use game_controller_msgs::{ControlMessage, HlControlMessage,};

/// This struct represents a sender for control messages. The messages are sent as UDP packets to
/// the given destination address. The states to be sent are obtained from a [tokio::sync::watch]
/// channel. This struct works both for sending to players and sending to monitor applications. The
/// caller can select what is desired by supplying an appropriate destination address and the flag.
pub struct RefereeSender {
    socket: UdpSocket,
    params: Params,
    game_receiver: watch::Receiver<Game>,
    to_monitor: bool,
}

impl RefereeSender {
    /// The period at which control messages are sent.
    const SEND_INTERVAL: Duration = Duration::from_millis(500);

    /// This function creates a new sender for control messages.
    pub async fn new(
        address: IpAddr,
        params: Params,
        game_receiver: watch::Receiver<Game>,
        to_monitor: bool,
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
                socket.set_broadcast(false)?;
                socket.connect((address, 4141)).await?;
                socket
            },
            params,
            game_receiver,
            to_monitor,
        })
    }

    /// This function runs the sender indefinitely.
    pub async fn run(&self) {
        let mut interval = interval(Self::SEND_INTERVAL);
        let mut packet_number: u8 = 0;
        loop {
            if self.params.competition.league == 0 {
                interval.tick().await;
                // It would be better if the timers were updated to the current time before
                // serialization. At the moment, this is not necessary because the main thread updates
                // the state each time a (rounded) timer crosses a second boundary. However, this tight
                // coupling between the fact that timers are sent as seconds and the main logic is
                // undesirable.
                let buffer: Bytes = ControlMessage::new(
                    &self.game_receiver.borrow(),
                    &self.params,
                    packet_number,
                    self.to_monitor,
                )
                .into();
                let _ = self.socket.send(&buffer).await;
                packet_number = packet_number.wrapping_add(1);
            } else if self.params.competition.league == 1 {
                interval.tick().await;
                // It would be better if the timers were updated to the current time before
                // serialization. At the moment, this is not necessary because the main thread updates
                // the state each time a (rounded) timer crosses a second boundary. However, this tight
                // coupling between the fact that timers are sent as seconds and the main logic is
                // undesirable.
                let buffer: Bytes = HlControlMessage::new(
                    &self.game_receiver.borrow(),
                    &self.params,
                    packet_number,
                    self.to_monitor,
                )
                .into();
                let _ = self.socket.send(&buffer).await;
                packet_number = packet_number.wrapping_add(1);
            }
            
        }
    }
}
