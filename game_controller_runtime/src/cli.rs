//! This module defines the command line interface.

use std::path::PathBuf;

use clap::Parser;

/// This struct defines the parser for the command line arguments.
#[derive(Parser)]
#[command(about, author, version)]
pub struct Args {
    /// Set the competition type.
    #[arg(long, short)]
    pub competition: Option<String>,
    /// Set whether this is a play-off (long) game.
    #[arg(long)]
    pub play_off: bool,
    /// Set the home team (name or number).
    #[arg(long)]
    pub home_team: Option<String>,
    /// Set the away team (name or number).
    #[arg(long)]
    pub away_team: Option<String>,
    /// Open the main window in fullscreen mode.
    #[arg(long, short)]
    pub fullscreen: bool,
    /// Set the network interface to listen/send on/to.
    #[arg(long, short)]
    pub interface: Option<String>,
    /// Send control messages to 255.255.255.255.
    #[arg(long, short)]
    pub broadcast: bool,
    /// Join multicast groups for simulated team communication.
    #[arg(long, short)]
    pub multicast: bool,
    /// Sync the log file to the storage device after each entry.
    #[arg(long)]
    pub sync: bool,
    /// Specify the path to a log file to replay.
    #[arg(long)]
    pub replay: Option<PathBuf>,
    /// the League the controller is for
    #[arg(long, short)]
    pub league: bool,
}
