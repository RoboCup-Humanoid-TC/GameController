//! This module implements functions to create statistics about general game events.

use std::{collections::HashMap, time::Duration};

use anyhow::{bail, Result};
use bytes::Bytes;
use enum_map::{enum_map, EnumMap};

use game_controller_core::{
    action::VAction,
    actions::{Goal, Penalize, StartSetPlay, Timeout, Undo},
    log::{LogEntry, LoggedAction, LoggedMetadata, TimestampedLogEntry},
    types::{
        ActionSource, Game, Params, Penalty, PenaltyCall, Phase, PlayerNumber, SetPlay, Side, State,
    },
};
use game_controller_msgs::StatusMessage;

/// This struct represents the statistics of a single team in a single game. Penalty shoot-outs
/// are generally not included.
#[derive(Default)]
pub struct Statistics {
    /// The number of goals a team has scored.
    goals: u32,
    /// The number of timeouts a team has taken.
    timeouts: u32,
    /// The number of penalties a team has been called for.
    penalties: EnumMap<PenaltyCall, u32>,
    /// The number of set plays that a team has been awarded.
    set_plays_for: EnumMap<SetPlay, u32>,
    /// The number of set plays that have been awarded against a team.
    set_plays_against: EnumMap<SetPlay, u32>,
    /// The sum over the durations that each player of this team has been active during the Ready,
    /// Set or Playing state.
    active_players: Duration,
    /// The duration in which the game was in the Ready, Set or Playing state.
    ready_set_playing: Duration,
    /// The duration in which the game was in the Playing state.
    playing: Duration,
}

/// This function creates statistics about general game events in a single game. For each team, a
/// line is written to the standard output with a number of comma separated values: the team
/// number, followed by a number of statistics that are also given in the [header] function.
pub fn evaluate(entries: Vec<TimestampedLogEntry>) -> Result<()> {
    let mut iter = entries.iter();
    let metadata: &LoggedMetadata =
        if let LogEntry::Metadata(metadata) = &iter.next().unwrap().entry {
            metadata
        } else {
            bail!("first log entry must be metadata");
        };
    let params: &Params = &metadata.params;
    let mut statistics = enum_map! {
        _ => Statistics::default(),
    };
    let mut actions: Vec<(Option<&Game>, &LoggedAction)> = vec![];
    let mut last_aliveness = HashMap::<(Side, PlayerNumber), Duration>::new();
    let mut last_state: Option<&Game> = None;
    let mut last_timestamp = Duration::ZERO;
    let mut last_stopped_timestamp = Duration::ZERO;
    actions.reserve(entries.len());
    iter.for_each(|entry| match &entry.entry {
        LogEntry::Action(action) => {
            if let VAction::Undo(Undo { states }) = action.action {
                let mut i = 0;
                while i < states {
                    if actions.pop().unwrap().1.source == ActionSource::User {
                        i += 1;
                    }
                }
            } else {
                actions.push((last_state, action));
            }
        }
        LogEntry::GameState(state) => {
            if let Some(last) = last_state {
                if last.phase != Phase::PenaltyShootout
                    && matches!(last.state, State::Ready | State::Playing | State::Set)
                {
                    let dt = entry.timestamp - last_timestamp;
                    for side in [Side::Home, Side::Away] {
                        let active_players = last.teams[side]
                            .players
                            .iter()
                            .zip(PlayerNumber::MIN..=PlayerNumber::MAX)
                            .filter(|(player, number)| {
                                player.penalty == Penalty::NoPenalty
                                    && last_aliveness
                                        .get(&(side, PlayerNumber::new(*number)))
                                        .map_or(false, |t| {
                                            *t + Duration::from_secs(4) >= last_stopped_timestamp
                                        })
                            })
                            .count() as u32;
                        statistics[side].active_players += dt * active_players;
                        statistics[side].ready_set_playing += dt;
                        if last.state == State::Playing {
                            statistics[side].playing += dt;
                        }
                    }
                } else {
                    last_stopped_timestamp = entry.timestamp;
                }
            }
            last_state = Some(state);
            last_timestamp = entry.timestamp;
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
        _ => {}
    });
    for (game, action) in &actions {
        if game.is_some_and(|game| game.phase == Phase::PenaltyShootout) {
            continue;
        }
        match action.action {
            VAction::Goal(Goal { side }) => {
                if !game.is_some_and(|game| game.teams[side].illegal_communication) {
                    statistics[side].goals += 1;
                }
            }
            VAction::Penalize(Penalize {
                side,
                player: _,
                call,
            }) => {
                statistics[side].penalties[call] += 1;
                match call {
                    PenaltyCall::Foul => {
                        statistics[-side].set_plays_for[SetPlay::PushingFreeKick] += 1;
                        statistics[side].set_plays_against[SetPlay::PushingFreeKick] += 1;
                    }
                    PenaltyCall::PenaltyKick => {
                        statistics[-side].set_plays_for[SetPlay::PenaltyKick] += 1;
                        statistics[side].set_plays_against[SetPlay::PenaltyKick] += 1;
                    }
                    _ => {}
                }
            }
            VAction::StartSetPlay(StartSetPlay { side, set_play }) => {
                statistics[side].set_plays_for[set_play] += 1;
                statistics[-side].set_plays_against[set_play] += 1;
            }
            VAction::Timeout(Timeout { side: Some(side) }) => {
                statistics[side].timeouts += 1;
            }
            VAction::Undo(_) => panic!("an undo action cannot occur here anymore"),
            _ => {}
        }
    }
    for side in [Side::Home, Side::Away] {
        println!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            params.game.teams[side].number,
            statistics[side].goals,
            statistics[side].timeouts,
            statistics[side].penalties[PenaltyCall::RequestForPickUp],
            statistics[side].penalties[PenaltyCall::IllegalPosition],
            statistics[side].penalties[PenaltyCall::MotionInSet],
            statistics[side].penalties[PenaltyCall::FallenInactive],
            statistics[side].penalties[PenaltyCall::LocalGameStuck],
            statistics[side].penalties[PenaltyCall::BallHolding],
            statistics[side].penalties[PenaltyCall::PlayerStance],
            statistics[side].penalties[PenaltyCall::Pushing]
                + statistics[side].penalties[PenaltyCall::Foul]
                + statistics[side].penalties[PenaltyCall::PenaltyKick],
            statistics[side].penalties[PenaltyCall::PlayingWithArmsHands],
            statistics[side].penalties[PenaltyCall::LeavingTheField],
            statistics[side].set_plays_against[SetPlay::KickIn],
            statistics[side].set_plays_against[SetPlay::GoalKick],
            statistics[side].set_plays_against[SetPlay::CornerKick],
            statistics[side].set_plays_against[SetPlay::PushingFreeKick],
            statistics[side].set_plays_against[SetPlay::PenaltyKick],
            statistics[side].set_plays_for[SetPlay::KickIn],
            statistics[side].set_plays_for[SetPlay::GoalKick],
            statistics[side].set_plays_for[SetPlay::CornerKick],
            statistics[side].set_plays_for[SetPlay::PushingFreeKick],
            statistics[side].set_plays_for[SetPlay::PenaltyKick],
            statistics[side].active_players.as_millis(),
            statistics[side].ready_set_playing.as_millis(),
            statistics[side].playing.as_millis(),
        );
    }
    Ok(())
}

/// This function writes a CSV header to the standard output that specifies the fields that
/// [evaluate] would write.
pub fn header() {
    println!(
        "team,goals,timeouts,request for pickup,illegal position,motion in set,\
        fallen/inactive,local game stuck,ball holding,player stance,pushing,\
        playing with arms/hands,leaving the field,kick-in against,goal kick against,\
        corner kick against,pushing free kick against,penalty kick against,kick-in for,\
        goal kick for,corner kick for,pushing free kick for,penalty kick for,playing,\
        active players,ready set playing"
    );
}
