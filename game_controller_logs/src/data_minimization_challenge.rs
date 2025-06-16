//! This module implements functions to evaluate the data minimization challenge (RoboCup 2023).

use std::{collections::HashMap, time::Duration};

use anyhow::{bail, Result};
use bytes::Bytes;
use enum_map::enum_map;

use game_controller_core::{
    log::{LogEntry, LoggedMetadata, TimestampedLogEntry},
    types::{Game, Params, Penalty, Phase, PlayerNumber, Side, State},
};
use game_controller_msgs::StatusMessage;

/// This function checks if the given game is in a state where team messages are counted for this
/// challenge.
fn is_valid_state(game: &Game) -> bool {
    game.phase != Phase::PenaltyShootout
        && matches!(game.state, State::Ready | State::Playing | State::Set)
}

/// This function evaluates the statistics for the data minimization challenge on a single game.
/// For each team, a line is written to the standard output with three comma separated values: the
/// team number, the number of payload bytes that the team sent during the game, and the overall
/// uptime of the team during the game in milliseconds.
pub fn evaluate(entries: Vec<TimestampedLogEntry>) -> Result<()> {
    let mut iter = entries.iter();
    let metadata: &LoggedMetadata =
        if let LogEntry::Metadata(metadata) = &iter.next().unwrap().entry {
            metadata
        } else {
            bail!("first log entry must be metadata");
        };
    let params: &Params = &metadata.params;
    let mut last_aliveness = HashMap::<(Side, PlayerNumber), Duration>::new();
    let mut stats = enum_map! {
        _ => (0usize, Duration::ZERO),
    };
    let mut last: Option<(&Game, Duration)> = None;
    // Timestamp of the last transition from initial/finished/timeout to ready/set/playing (at
    // least if the current state is ready/set/playing).
    let mut last_stopped_timestamp = Duration::ZERO;
    for entry in iter {
        match &entry.entry {
            LogEntry::GameState(state) => {
                if let Some((last_state, last_timestamp)) = last {
                    if is_valid_state(last_state) {
                        let dt = entry.timestamp - last_timestamp;
                        for side in [Side::Home, Side::Away] {
                            // A player counts as being alive if it is
                            // - not penalized AND
                            // - has sent a status message during this segment of ready/set/playing
                            // (- 4 seconds because this is the minimum frequency of status
                            // messages, but the state segment could be shorter than that).
                            let active_players = last_state.teams[side]
                                .players
                                .iter()
                                .zip(PlayerNumber::MIN..=PlayerNumber::MAX)
                                .filter(|(player, number)| {
                                    player.penalty == Penalty::NoPenalty
                                        && last_aliveness
                                            .get(&(side, PlayerNumber::new(*number)))
                                            .map_or(false, |t| {
                                                *t + Duration::from_secs(4)
                                                    >= last_stopped_timestamp
                                            })
                                })
                                .count() as u32;
                            stats[side].1 += dt * active_players;
                        }
                    } else {
                        last_stopped_timestamp = entry.timestamp;
                    }
                }
                last = Some((state, entry.timestamp));
            }
            LogEntry::StatusMessage(status_message) => {
                if let Ok(status_message) =
                    StatusMessage::try_from(Bytes::from(status_message.data.clone()))
                {
                    if let Some(side) = params.game.get_side(status_message.team_number) {
                        last_aliveness.insert(
                            (side, PlayerNumber::new(status_message.player_number)),
                            entry.timestamp,
                        );
                    }
                }
            }
            LogEntry::TeamMessage(team_message) => {
                if let Some((last_state, _)) = last {
                    if is_valid_state(last_state) {
                        if let Some(side) = params.game.get_side(team_message.team) {
                            stats[side].0 += team_message.data.len();
                        }
                    }
                }
            }
            _ => {}
        }
    }
    for side in [Side::Home, Side::Away] {
        println!(
            "{},{},{}",
            params.game.teams[side].number,
            stats[side].0,
            stats[side].1.as_millis()
        );
    }
    Ok(())
}
