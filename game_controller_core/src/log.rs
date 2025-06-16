//! This module defines structures that can be logged, a trait for loggers and an implementation
//! that just saves entries in memory.

use std::{net::IpAddr, time::Duration};

use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::action::VAction;
use crate::types::{ActionSource, Game, Params};

/// This struct defines an entry type that should appear once at the beginning of a log file.
#[serde_as]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggedMetadata {
    /// The name of the program that created this log.
    pub creator: String,
    /// The version of the program that created this log.
    pub version: u32,
    /// The "real" time when this log was created.
    #[serde_as(as = "Rfc3339")]
    pub timestamp: OffsetDateTime,
    /// The combined parameters.
    pub params: Box<Params>,
}

/// This struct defines an entry type that represents an action that is applied to the game.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggedAction {
    /// The type of event which triggered the action.
    pub source: ActionSource,
    /// The action itself.
    pub action: VAction,
}

/// This struct defines an entry type with the complete description of the dynamic game state. This
/// is stored on the heap because it is much larger than the other log entries.
pub type LoggedGameState = Box<Game>;

/// This struct defines an entry type for a received monitor request.
#[serde_as]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggedMonitorRequest {
    /// The host which sent the monitor request.
    pub host: IpAddr,
    /// The binary data of the monitor request.
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
}

/// This struct defines an entry type for a received status message.
#[serde_as]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggedStatusMessage {
    /// The host which sent the status message.
    pub host: IpAddr,
    /// The binary data of the status message.
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
}

/// This struct defines an entry type for a received team message.
#[serde_as]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggedTeamMessage {
    /// The team number of the team which sent the team message.
    pub team: u8,
    /// The host which sent the team message.
    pub host: IpAddr,
    /// The binary data of the team message.
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LogEntry {
    Metadata(LoggedMetadata),
    Action(LoggedAction),
    GameState(LoggedGameState),
    MonitorRequest(LoggedMonitorRequest),
    StatusMessage(LoggedStatusMessage),
    TeamMessage(LoggedTeamMessage),
    /// This is an marker that is the last entry in intact log files and allows to reconstruct the
    /// final state of timers.
    End,
}

/// This struct wraps a log entry together with a timestamp.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimestampedLogEntry {
    /// The timestamp of the entry as its duration since the start of the game.
    pub timestamp: Duration,
    /// The log entry itself.
    pub entry: LogEntry,
}

/// This trait must be implmented by logging methods.
pub trait Logger {
    /// This function appends an entry to the log.
    fn append(&mut self, entry: TimestampedLogEntry);
}

/// This struct defines a logger that does nothing.
pub struct NullLogger;

impl Logger for NullLogger {
    fn append(&mut self, _entry: TimestampedLogEntry) {}
}
