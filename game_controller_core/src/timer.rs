//! This module defines types for timers that count down during a game. It is a bit awkward to use
//! both [std::time::Duration] and [time::Duration] (the latter aliased as [SignedDuration]). The
//! reason is that there are some timestamps that can be negative and others that can't.

use std::{cmp::min, mem::take, ops::Index, time::Duration};

use serde::{Deserialize, Serialize};
pub use time::Duration as SignedDuration;

use crate::action::VAction;
use crate::types::{Game, Params, Phase, State, SecState};

/// This enumerates conditions which restrict in which states a timer actually counts down.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RunCondition {
    /// The timer counts down in any state.
    Always,
    /// The timer counts down during Playing, but also during Ready and Set if the game is not a
    /// "long" game.
    Playing,
    /// The timer counts during the Ready and Playing states.
    ReadyOrPlaying,
}

/// This struct can be queried for values of each run condition. It mainly exists because there are
/// technical reasons that the conditions can not be evaluated directly in [Timer::seek] or
/// [Timer::is_running].
pub struct EvaluatedRunConditions {
    playing: bool,
    ready_or_playing: bool,
}

impl EvaluatedRunConditions {
    /// This function evaluates the run conditions in a given game state so they can be queried
    /// later.
    pub fn new(game: &Game, params: &Params) -> Self {
        Self {
            playing: (game.state == State::Playing
                || ((game.state == State::Ready || game.state == State::Set)
                    && game.phase != Phase::PenaltyShootout
                    && !params.game.long
                    && game.primary_timer.get_remaining()
                        != TryInto::<SignedDuration>::try_into(params.competition.half_duration)
                            .unwrap())) && (game.sec_state.state == SecState::Normal || (game.sec_state.state == SecState::Penalityshoot && game.state != State::Set)),
            ready_or_playing: (game.state == State::Ready || game.state == State::Playing) && game.sec_state.state == SecState::Normal,
        }
    }
}

impl Index<RunCondition> for EvaluatedRunConditions {
    type Output = bool;

    fn index(&self, index: RunCondition) -> &Self::Output {
        match index {
            RunCondition::Always => &true,
            RunCondition::Playing => &self.playing,
            RunCondition::ReadyOrPlaying => &self.ready_or_playing,
        }
    }
}

/// This enumerates the possible behaviors of a timer when 0 is reached.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BehaviorAtZero {
    /// When the timer reaches 0, it stops itself and potentially releases some actions to be
    /// executed.
    Expire(Vec<VAction>),
    /// The timer is clipped at 0 but stays "active".
    Clip,
    /// The timer continues to run into negative durations.
    Overflow,
}

/// This struct describes the state of a timer. A timer can be either started or stopped.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Timer {
    /// The timer is active with some parameters. It may still be paused.
    #[serde(rename_all = "camelCase")]
    Started {
        /// The remaining time until the timer reaches 0.
        remaining: SignedDuration,
        /// The condition under which the timer actually counts down.
        run_condition: RunCondition,
        /// What happens when the timer reaches 0.
        behavior_at_zero: BehaviorAtZero,
    },
    /// The timer is currently not in use.
    #[default]
    Stopped,
}

impl Timer {
    /// This function lets time progress. The caller must supply the current state of the run
    /// conditions. The caller must also ensure that if the timer's behavior is set to
    /// [BehaviorAtZero::Expire], the requested duration can be at most the remaining time. When
    /// such a timer reaches 0, this function releases the stored actions as result.
    pub fn seek(
        &mut self,
        dt: Duration,
        run_conditions: &EvaluatedRunConditions,
    ) -> Option<Vec<VAction>> {
        match self {
            Self::Started {
                remaining,
                run_condition,
                behavior_at_zero,
            } => {
                if run_conditions[*run_condition] {
                    match behavior_at_zero {
                        BehaviorAtZero::Expire(actions) => {
                            if dt > *remaining {
                                panic!("timers that expire can't be sought beyond their expiration ({:?}, {:?})", dt, *remaining);
                            }
                            *remaining -= dt;
                            if remaining.is_zero() {
                                let result = take(actions);
                                *self = Self::Stopped;
                                Some(result)
                            } else {
                                None
                            }
                        }
                        BehaviorAtZero::Clip => {
                            *remaining -= min(*remaining, dt.try_into().unwrap());
                            None
                        }
                        BehaviorAtZero::Overflow => {
                            *remaining -= dt;
                            None
                        }
                    }
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    /// This function returns if the timer will count down if [Timer::seek] is called. The caller
    /// must supply the current state of the run conditions.
    pub fn is_running(&self, run_conditions: &EvaluatedRunConditions) -> bool {
        match self {
            Self::Started {
                remaining,
                run_condition,
                behavior_at_zero,
            } => {
                run_conditions[*run_condition]
                    && !(remaining.is_zero() && matches!(behavior_at_zero, BehaviorAtZero::Clip))
            }
            _ => false,
        }
    }

    pub fn timer_to_zero(&mut self) {
        if let Self::Started { remaining, .. } = self {
            *remaining = SignedDuration::ZERO;
        }
    }

    /// This function returns if the timer will expire at some point in the future.
    pub fn will_expire(&self) -> bool {
        matches!(
            self,
            Self::Started {
                behavior_at_zero: BehaviorAtZero::Expire(_),
                ..
            }
        )
    }

    /// This function returns the time remaining until the timer reaches 0 (can be negative).
    pub fn get_remaining(&self) -> SignedDuration {
        match self {
            Self::Started { remaining, .. } => *remaining,
            _ => SignedDuration::ZERO,
        }
    }
}
