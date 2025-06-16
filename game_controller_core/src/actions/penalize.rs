use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext, VAction};
use crate::actions::{StartSetPlay, Unpenalize};
use crate::timer::{BehaviorAtZero, RunCondition, SignedDuration, Timer};
use crate::types::{Penalty, PenaltyCall, Phase, PlayerNumber, SetPlay, Side, State};

/// This struct defines an action to apply a penalty to players.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Penalize {
    /// The side whose player is penalized.
    pub side: Side,
    /// The number of the player who is penalized.
    pub player: PlayerNumber,
    /// The penalty which has been called for the player.
    pub call: PenaltyCall,
}

impl Action for Penalize {
    fn execute(&self, c: &mut ActionContext) {
        // Map the penalty call to a penalty.
        let penalty = match self.call {
            PenaltyCall::RequestForPickUp => Penalty::PickedUp,
            PenaltyCall::IllegalPosition => {
                if c.game.state == State::Set {
                    Penalty::IllegalPositionInSet
                } else {
                    Penalty::IllegalPosition
                }
            }
            PenaltyCall::MotionInSet => Penalty::MotionInSet,
            PenaltyCall::FallenInactive => Penalty::FallenInactive,
            PenaltyCall::LocalGameStuck => Penalty::LocalGameStuck,
            PenaltyCall::BallHolding => Penalty::BallHolding,
            PenaltyCall::PlayerStance => Penalty::PlayerStance,
            PenaltyCall::Pushing => Penalty::PlayerPushing,
            PenaltyCall::Foul => Penalty::PlayerPushing,
            PenaltyCall::PenaltyKick => Penalty::PlayerPushing,
            PenaltyCall::PlayingWithArmsHands => Penalty::PlayingWithArmsHands,
            PenaltyCall::LeavingTheField => Penalty::LeavingTheField,
        };

        c.game.teams[self.side][self.player].penalty_timer = if penalty == Penalty::PickedUp
            && matches!(
                c.game.state,
                State::Initial | State::Finished | State::Timeout
            ) {
            // Picking up a player does not start a timer in "halted" game states.
            Timer::Stopped
        } else {
            Timer::Started {
                remaining: ({
                    // The duration is composed of the base duration plus the increment for each
                    // previous incremental penalty of this team.
                    let duration = c.params.competition.penalties[penalty].duration
                        + if c.params.competition.penalties[penalty].incremental {
                            c.params.competition.penalty_duration_increment
                                * c.game.teams[self.side].penalty_counter
                        } else {
                            Duration::ZERO
                        };
                    let previous_penalty = c.game.teams[self.side][self.player].penalty;
                    if penalty == Penalty::PickedUp && previous_penalty != Penalty::NoPenalty {
                        // Picking up a player in other states should keep the previous timer if
                        // the player was  already penalized, but enforce that the total penalty
                        // time is at least that of the pick-up penalty.
                        let extra_penalty_duration = TryInto::<SignedDuration>::try_into(duration)
                            .unwrap()
                            - c.params.competition.penalties[previous_penalty].duration;
                        c.game.teams[self.side][self.player]
                            .penalty_timer
                            .get_remaining()
                            + if extra_penalty_duration.is_positive() {
                                // If any penalty that is shorter than pick-up is incremental, we
                                // would have to save how long the duration *actually* was. I
                                // don't want to introduce extra complexity only for this special
                                // case as long as it isn't necessary.
                                assert!(
                                    !c.params.competition.penalties[previous_penalty].incremental
                                );
                                extra_penalty_duration
                            } else {
                                SignedDuration::ZERO
                            }
                    } else {
                        duration.try_into().unwrap()
                    }
                }),
                run_condition: RunCondition::ReadyOrPlaying,
                // Motion in Set is removed automatically.
                behavior_at_zero: if penalty == Penalty::MotionInSet {
                    BehaviorAtZero::Expire(vec![VAction::Unpenalize(Unpenalize {
                        side: self.side,
                        player: self.player,
                    })])
                } else if penalty == Penalty::PickedUp {
                    BehaviorAtZero::Expire(vec![])
                } else {
                    BehaviorAtZero::Clip
                },
            }
        };

        c.game.teams[self.side][self.player].penalty = penalty;
        if c.params.competition.penalties[penalty].incremental {
            c.game.teams[self.side].penalty_counter += 1;
        }

        // If this call requires switching to a set play, it is started here.
        if let Some(set_play) = match self.call {
            PenaltyCall::Foul => Some(SetPlay::PushingFreeKick),
            PenaltyCall::PenaltyKick => Some(SetPlay::PenaltyKick),
            _ => None,
        } {
            StartSetPlay {
                side: -self.side,
                set_play,
            }
            .execute(c);
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        (c.game.teams[self.side][self.player].penalty == Penalty::NoPenalty
            || (self.call == PenaltyCall::RequestForPickUp
                && c.game.teams[self.side][self.player].penalty != Penalty::PickedUp
                && c.game.teams[self.side][self.player].penalty != Penalty::Substitute))
            && (match self.call {
                PenaltyCall::RequestForPickUp => true,
                PenaltyCall::IllegalPosition => {
                    c.game.phase != Phase::PenaltyShootout
                        && (c.game.state == State::Ready // Not possible in this state, but can
                                                         // happen if it happens shortly before a
                                                         // goal and the GameController presses the
                                                         // goal first.
                            || c.game.state == State::Set
                            || c.game.state == State::Playing)
                }
                PenaltyCall::MotionInSet => c.game.state == State::Set,
                PenaltyCall::FallenInactive => {
                    c.game.state == State::Ready
                        || c.game.state == State::Set
                        || c.game.state == State::Playing
                }
                PenaltyCall::LocalGameStuck => {
                    c.game.phase != Phase::PenaltyShootout && c.game.state == State::Playing
                }
                PenaltyCall::BallHolding => {
                    c.game.state == State::Ready // Not possible in this state, but can happen in
                                                 // Playing shortly before a goal and the
                                                 // GameController operator clicks the goal first.
                        || c.game.state == State::Playing
                }
                PenaltyCall::PlayerStance => {
                    c.game.state == State::Ready
                        || c.game.state == State::Set
                        || c.game.state == State::Playing
                }
                PenaltyCall::Pushing => {
                    c.game.state == State::Ready
                        || c.game.state == State::Set // Not possible in this state, but can happen
                                                      // in Ready shortly before the timer expires.
                        || c.game.state == State::Playing
                }
                PenaltyCall::Foul => {
                    c.game.phase != Phase::PenaltyShootout
                        && c.game.state == State::Playing
                        && c.game.set_play == SetPlay::NoSetPlay
                        && c.params.competition.challenge_mode.is_none()
                }
                PenaltyCall::PenaltyKick => {
                    c.game.phase != Phase::PenaltyShootout
                        && c.game.state == State::Playing
                        && c.game.set_play == SetPlay::NoSetPlay
                        && c.params.competition.challenge_mode.is_none()
                }
                PenaltyCall::PlayingWithArmsHands => {
                    c.game.state == State::Ready // Not possible in this state, but can happen in
                                                 // Playing shortly before a goal and the
                                                 // GameController oprtator clicks the goal first.
                        || c.game.state == State::Playing
                }
                PenaltyCall::LeavingTheField => {
                    c.game.state == State::Ready
                        || c.game.state == State::Set // Not possible in this state, but can happen
                                                      // in Ready shortly before the timer expires.
                        || c.game.state == State::Playing
                }
            })
    }
}
