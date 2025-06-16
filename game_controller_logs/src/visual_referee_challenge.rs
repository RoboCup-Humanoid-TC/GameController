//! This module implements functions to evaluate the visual referee challenge (RoboCup 2023).

use std::time::Duration;

use anyhow::{bail, Result};
use bytes::Bytes;

use game_controller_core::{
    action::VAction,
    actions::Undo,
    log::{LogEntry, LoggedAction, LoggedMetadata, TimestampedLogEntry},
    types::{ActionSource, Params},
};
use game_controller_msgs::VrcMessage;

/// This function extracts the reported signals for the visual referee challenge from a single
/// game. For each valid report, a line is written to the standard output with four comma separated
/// values: the time in milliseconds since the log was started, the team number of the reporting
/// player, the player number of the reporting player, and the reported gesture (as number). Game
/// events after which a signal should have been shown are entries with team number and player
/// number set to -1, and the gesture field is replaced by a string identifying the game event.
/// Note that mercy rule goals that end the game are included as goals, not end of half. This has
/// to be manually adjusted in the exported files in order to use the correct weightings. Actions
/// that have been undone are exported, too, because one can then manually inspect when and if a
/// signal was actually shown.
pub fn evaluate(entries: Vec<TimestampedLogEntry>) -> Result<()> {
    let mut iter = entries.iter();
    let metadata: &LoggedMetadata =
        if let LogEntry::Metadata(metadata) = &iter.next().unwrap().entry {
            metadata
        } else {
            bail!("first log entry must be metadata");
        };
    let params: &Params = &metadata.params;
    let mut actions: Vec<(Duration, &LoggedAction, bool)> = vec![];
    iter.for_each(|entry| match &entry.entry {
        LogEntry::Action(action) => {
            if let VAction::Undo(Undo { states }) = action.action {
                let mut index: usize = actions.len() - 1;
                let mut i = 0;
                while i < states {
                    let was_undone = actions[index].2;
                    actions[index].2 = true;
                    if !was_undone && actions[index].1.source == ActionSource::User {
                        i += 1;
                    }
                    index -= 1;
                }
                actions.push((entry.timestamp, action, true));
            } else {
                actions.push((entry.timestamp, action, false));
            }
        }
        LogEntry::StatusMessage(status_message) => {
            if let Ok(vrc_message) = VrcMessage::try_from(Bytes::from(status_message.data.clone()))
            {
                if params.game.get_side(vrc_message.team_number).is_some() {
                    println!(
                        "{},{},{},{}",
                        entry.timestamp.as_millis(),
                        vrc_message.team_number,
                        vrc_message.player_number,
                        vrc_message.gesture,
                    );
                }
            }
        }
        _ => {}
    });

    for action in actions {
        if action.1.source != ActionSource::User {
            continue;
        }
        let name = match &action.1.action {
            VAction::FinishHalf(_) => "endOfHalf",
            VAction::FreeSetPlay(_) => "kickOff",
            VAction::Goal(_) => "goal",
            _ => continue,
        };
        let name_postfixed = format!("{}{}", name, if action.2 { "-undone" } else { "" });
        println!("{},{},{},{}", action.0.as_millis(), -1, -1, name_postfixed);
    }

    Ok(())
}
