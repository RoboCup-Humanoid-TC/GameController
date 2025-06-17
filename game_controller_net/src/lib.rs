//! This crate contains network services for the GameController.

use std::net::IpAddr;

use bytes::Bytes;

mod control_message_sender;
mod monitor_request_receiver;
mod referee_receiver;
mod referee_sender;
mod status_message_forwarder;
mod status_message_receiver;
mod team_message_receiver;
mod workaround;

pub use control_message_sender::ControlMessageSender;
pub use monitor_request_receiver::MonitorRequestReceiver;
pub use referee_receiver::RefereeReceiver;
pub use referee_sender::RefereeSender;
pub use status_message_forwarder::StatusMessageForwarder;
pub use status_message_receiver::StatusMessageReceiver;
pub use team_message_receiver::TeamMessageReceiver;

/// This enumerates network events.
#[derive(Debug)]
pub enum Event {
    /// An incoming monitor request.
    MonitorRequest {
        /// The host which sent the request.
        host: IpAddr,
        /// The payload of the request.
        data: Bytes,
        /// Whether there would have been more bytes in the request.
        too_long: bool,
    },
    /// An incoming status message (from a player).
    StatusMessage {
        /// The host which sent the message.
        host: IpAddr,
        /// The payload of the message.
        data: Bytes,
        /// Whether there would have been more bytes in the message.
        too_long: bool,
    },
    /// An incoming team message (from a player).
    TeamMessage {
        /// The host which sent the message.
        host: IpAddr,
        /// The team which sent the message.
        team: u8,
        /// The payload of the message.
        data: Bytes,
        /// Whether there would have been more bytes in the message.
        too_long: bool,
    },
    RefereeMessage {
        /// The payload of the message.
        data: Bytes,
    },
}
