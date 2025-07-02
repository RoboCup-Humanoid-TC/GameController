//! This crate defines the main program to analyze GameController log files.

use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use game_controller_core::log::TimestampedLogEntry;

use game_controller_logs::{statistics, team_communication};

/// This struct defines the parser for the command line arguments.
#[derive(Parser)]
#[command(about, author, version)]
struct Args {
    /// The path of the log file to analyze.
    #[arg(long, short)]
    pub path: Option<PathBuf>,
    /// The kind of thing that should be done with that log file.
    #[command(subcommand)]
    pub command: Commands,
}

/// This struct defines the command line subcommands.
#[derive(Subcommand)]
enum Commands {
    /// Extract statistics about general game events.
    Statistics,
    /// Extract statistics about the bandwidth usage of team communication.
    TeamCommunication,
}

/// This function applies a subcommand to one log file.
fn process_file(f: File, command: &Commands) -> Result<()> {
    let entries: Vec<TimestampedLogEntry> =
        serde_yaml::from_reader(f).context("could not parse log file")?;
    match command {
        Commands::Statistics => {
            statistics::evaluate(entries).context("could not create statistics from log file")?;
        }
        Commands::TeamCommunication => {
            team_communication::evaluate(entries)
                .context("could not evaluate team communication")?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(path) = args.path {
        let f = File::open(path).context("could not open log file")?;
        process_file(f, &args.command)?;
    }

    Ok(())
}
