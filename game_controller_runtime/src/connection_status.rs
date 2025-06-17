//! This module defines utilities to manage the connection status of players.

use std::{collections::HashMap, time::Duration};

use enum_map::{enum_map, EnumMap};
use serde_repr::Serialize_repr;
use tokio::time::Instant;

use game_controller_core::types::{PlayerNumber, Side};

/// This enumerates the possible values of a player's connection status.
#[derive(Clone, Copy, Serialize_repr)]
#[repr(u8)]
pub enum ConnectionStatus {
    /// The player hasn't sent a status message for a long time and is probably not running.
    Offline = 0,
    /// The player has sent a status message, but it has been a while.
    Bad = 1,
    /// The player has sent a status message recently.
    Good = 2,
}

/// The upper bound on the time since the last status message for a good (but not yet bad)
/// connection status.
const CONNECTION_STATUS_TIMEOUT_GOOD: Duration = Duration::from_secs(2);

/// The upper bound on the time since the last status message for a bad (but not yet offline)
/// connection status.
const CONNECTION_STATUS_TIMEOUT_BAD: Duration = Duration::from_secs(4);

/// This type aliases a "two-dimensional array"-like map from players to connection status values.
pub type ConnectionStatusMap =
    EnumMap<Side, [ConnectionStatus; (PlayerNumber::MAX - PlayerNumber::MIN + 1) as usize]>;

/// This type aliases a hashmap from players (represented as pairs of a side and a player number)
/// to the timestamp when the last status message was received.
pub type AlivenessTimestampMap = HashMap<(Side, PlayerNumber), Instant>;

/// This function transforms a map from players to timestamps into a map of connection status
/// values, given the current time.
pub fn get_connection_status_map(
    timestamps: &AlivenessTimestampMap,
    now: &Instant,
) -> ConnectionStatusMap {
    let mut result = enum_map! {
        _ => [ConnectionStatus::Offline; (PlayerNumber::MAX - PlayerNumber::MIN + 1) as usize]
    };
    for (key, value) in timestamps {
        let time_since_alive = now.duration_since(*value);
        let status = if time_since_alive <= CONNECTION_STATUS_TIMEOUT_GOOD {
            ConnectionStatus::Good
        } else if time_since_alive <= CONNECTION_STATUS_TIMEOUT_BAD {
            ConnectionStatus::Bad
        } else {
            ConnectionStatus::Offline
        };
        result[key.0][(u8::from(key.1) - PlayerNumber::MIN) as usize] = status;
    }
    result
}

/// This function returns the duration until any player's connection status changes or [None] if
/// that will never happen.
pub fn get_next_connection_status_change(
    timestamps: &AlivenessTimestampMap,
    now: &Instant,
) -> Option<Duration> {
    timestamps
        .values()
        .flat_map(|timestamp| {
            if *timestamp + CONNECTION_STATUS_TIMEOUT_GOOD > *now {
                Some(*timestamp + CONNECTION_STATUS_TIMEOUT_GOOD - *now)
            } else if *timestamp + CONNECTION_STATUS_TIMEOUT_BAD > *now {
                Some(*timestamp + CONNECTION_STATUS_TIMEOUT_BAD - *now)
            } else {
                None
            }
        })
        .min()
}
