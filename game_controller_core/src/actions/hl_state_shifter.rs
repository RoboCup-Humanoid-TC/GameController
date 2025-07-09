use crate::action::{Action, ActionContext, VAction};
use crate::timer::{BehaviorAtZero, RunCondition, SignedDuration, Timer};
use crate::types::{Penalty, Phase, SecState, SecondaryState, SetPlay, Side, State};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlStateShifter {
    /// The type of set play to start.
    pub state: State,
}

impl Action for HlStateShifter {
    fn execute(&self, c: &mut ActionContext) {
        if self.state == State::Initial {
            // half time switch
            c.game.sides = -c.game.sides;
            match c.game.phase 
            {
                Phase::FirstHalf => c.game.phase = Phase::SecondHalf,
                Phase::SecondHalf => c.game.phase = Phase::FirstExtraHalf,
                Phase::FirstExtraHalf => c.game.phase = Phase::SecondExtraHalf,
                Phase::SecondExtraHalf => c.game.phase = Phase::PenaltyShootout,
                _ => {}
            }
            c.game.state = self.state;
            c.game.sec_state.state = SecState::Normal;
            c.game.kicking_side = Some(-c.game.kicking_side.unwrap());

            c.game.teams.values_mut().for_each(|team| {
                team.goalkeeper = None;
                team.penalty_shot = 0;
                team.penalty_shot_mask = 0;
                team.timeout_budget = c.params.competition.timeouts_per_team;
                team.players.iter_mut().for_each(|player| {
                    if player.penalty != Penalty::Substitute {
                        player.penalty = Penalty::NoPenalty;
                        player.penalty_timer = Timer::Stopped;
                    }
                });
            });

            c.game.secondary_timer = Timer::Stopped;
            if c.game.phase == Phase::FirstHalf || 
            c.game.phase == Phase::SecondHalf
            {
                c.game.primary_timer = Timer::Started {
                    remaining: c.params.competition.half_duration
                    .try_into()
                    .unwrap(),
                    run_condition: RunCondition::Playing,
                    behavior_at_zero: BehaviorAtZero::Overflow,
                };
            } 
            else
            {
                c.game.primary_timer = Timer::Started {
                    remaining: (c.params.competition.half_duration_overtime
                    .try_into()
                    .unwrap()),
                    run_condition: RunCondition::Playing,
                    behavior_at_zero: BehaviorAtZero::Overflow,
                };
            } 
        } else if self.state == State::Set && c.game.sec_state.state == SecState::Penaltyshoot {
            c.game.state = self.state;
            c.game.primary_timer = Timer::Started {
                remaining: c.params.competition
                .penalty_shot_duration
                .try_into()
                .unwrap(),
                run_condition: RunCondition::Playing,
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::HlStateShifter(
                    HlStateShifter { state: State::Set },
                )]),
            };
            c.game.teams.values_mut().for_each(|team| {
                team.goalkeeper = None;
                team.penalty_shot = 0;
                team.penalty_shot_mask = 0;
                team.players.iter_mut().for_each(|player| {
                    player.penalty = Penalty::Substitute;
                    player.penalty_timer = Timer::Stopped;
                });
            });
        } else if self.state == State::Playing && c.game.sec_state.state == SecState::Penaltyshoot {
            c.game.state = self.state;
        } else if self.state == State::Finished && c.game.sec_state.state == SecState::Penaltyshoot
        {
            c.game.state = State::Ready;
            c.game.primary_timer = Timer::Started {
                remaining: c.params.competition
                .penalty_shot_duration
                .try_into()
                .unwrap(),
                run_condition: RunCondition::Playing,
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::HlStateShifter(
                    HlStateShifter { state: State::Set },
                )]),
            };
            c.game.teams.values_mut().for_each(|team| {
                team.goalkeeper = None;
                team.players.iter_mut().for_each(|player| {
                    player.penalty = Penalty::Substitute;
                    player.penalty_timer = Timer::Stopped;
                });
            });
            c.game.sides = -c.game.sides;
            c.game.state = State::Set;
        } else if self.state == State::Ready {
            c.game.state = self.state;
            c.game.sec_state.state = SecState::Normal;
            c.game.secondary_timer = Timer::Started {
                remaining: SignedDuration::new(45, 0),
                run_condition: RunCondition::Always,
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::HlStateShifter(
                    HlStateShifter { state: State::Set },
                )]),
            };
        } else if self.state == State::Set {
            c.game.state = self.state;
            c.game.sec_state.state = SecState::Normal;
            c.game.secondary_timer = Timer::Stopped;
        } else if self.state == State::Playing {
            c.game.state = self.state;
            c.game.sec_state.state = SecState::Normal;
            c.game.secondary_timer = Timer::Started {
                remaining: SignedDuration::new(10, 0),
                run_condition: RunCondition::Always,
                behavior_at_zero: BehaviorAtZero::Expire(vec![]),
            };
            c.game.set_play = SetPlay::NoSetPlay;
        } else if self.state == State::Finished {
            c.game.state = self.state;
            c.game.sec_state.state = SecState::Normal;
            c.game.secondary_timer = Timer::Stopped;
            if c.game.phase != Phase::SecondExtraHalf
            {
                c.game.primary_timer = Timer::Started {
                    remaining: c.params.competition
                    .half_time_break_duration
                    .try_into()
                    .unwrap(),
                    run_condition: RunCondition::Always,
                    behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::HlStateShifter(
                        HlStateShifter {
                            state: State::Initial,
                        },
                    )]),
                };
            } 
            else
            {
                c.game.primary_timer = Timer::Stopped;
            }
            c.game.sec_state = SecondaryState {
                state: SecState::Normal,
                side: Side::None,
                phase: 0,
            };
        } else {
            c.game.state = State::Initial;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        if self.state == c.game.state {
            false
        } else if self.state == State::Initial
            && c.game.state == State::Finished
        {
            true
        } else if self.state == State::Ready && c.game.state == State::Initial {
            true
        } else if self.state == State::Set
            && (c.game.state == State::Ready
                || c.game.state == State::Initial)
        {
            true
        } else if self.state == State::Playing && c.game.state == State::Set {
            true
        } else if self.state == State::Finished && (c.game.state == State::Playing) {
            true
        } else if self.state == State::Timeout && 
        (c.game.state == State::Playing || c.game.state == State::Ready 
            || c.game.state == State::Set) {
            true
        } else {
            false
        }
    }
}
