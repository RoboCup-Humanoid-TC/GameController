use crate::action::{Action, ActionContext, VAction};
use crate::timer::{BehaviorAtZero, RunCondition, SignedDuration, Timer};
use crate::types::{Penalty, Phase, SetPlay, Side, State};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlStateShifter {
    /// The type of set play to start.
    pub state: State,
}

impl Action for HlStateShifter {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.phase == Phase::PenaltyShootout {
            if self.state == State::Set {
                if c.game.state == State::Finished {
                    c.game.teams.values_mut().for_each(|team| {
                        team.goalkeeper = None;
                        team.players.iter_mut().for_each(|player| {
                            player.penalty = Penalty::Substitute;
                            player.penalty_timer = Timer::Stopped;
                        });
                    });

                    c.game.sides = -c.game.sides;
                    c.game.kicking_side = c.game.kicking_side.map(|side| -side);
                }

                c.game.state = self.state;
                c.game.primary_timer = Timer::Started {
                    remaining: c
                        .params
                        .competition
                        .penalty_shot_duration
                        .try_into()
                        .unwrap(),
                    run_condition: RunCondition::Playing,
                    behavior_at_zero: BehaviorAtZero::Overflow,
                };
                c.game.secondary_timer = Timer::Stopped; // This can be set from a previous timeout.

                if let Some(side) = c.game.kicking_side {
                    c.game.teams[side].penalty_shot += 1;
                }
            } else if self.state == State::Playing {
                c.game.state = self.state;
            } else if self.state == State::Finished {
                c.game.state = self.state;
            }
        } else {
            if self.state == State::Initial {
                // half time switch
                c.game.sides = -c.game.sides;
                match c.game.phase {
                    Phase::FirstHalf => c.game.phase = Phase::SecondHalf,
                    Phase::SecondHalf => c.game.phase = Phase::FirstExtraHalf,
                    Phase::FirstExtraHalf => c.game.phase = Phase::SecondExtraHalf,
                    Phase::SecondExtraHalf => c.game.phase = Phase::PenaltyShootout,
                    _ => {}
                }
                c.game.state = self.state;
                c.game.set_play = SetPlay::NoSetPlay;
                c.game.kicking_side = c.game.kicking_side.map(|side| -side);
                match c.game.phase {
                    Phase::FirstHalf | Phase::FirstExtraHalf | Phase::PenaltyShootout => {
                        c.game.kicking_side = Some(c.params.game.kick_off_side)
                    }
                    Phase::SecondHalf | Phase::SecondExtraHalf => {
                        c.game.kicking_side = Some(-c.params.game.kick_off_side)
                    }
                    _ => {}
                }

                c.game.teams.values_mut().for_each(|team| {
                    team.timeout_budget = c.params.competition.timeouts_per_team;
                    team.players.iter_mut().for_each(|player| {
                        if player.penalty != Penalty::Substitute {
                            player.penalty = Penalty::NoPenalty;
                            player.penalty_timer = Timer::Stopped;
                        }
                    });
                });

                if c.game.phase == Phase::FirstHalf || c.game.phase == Phase::SecondHalf {
                    c.game.primary_timer = Timer::Started {
                        remaining: c.params.competition.half_duration.try_into().unwrap(),
                        run_condition: RunCondition::Playing,
                        behavior_at_zero: BehaviorAtZero::Overflow,
                    };
                } else {
                    c.game.primary_timer = Timer::Started {
                        remaining: (c
                            .params
                            .competition
                            .half_duration_overtime
                            .try_into()
                            .unwrap()),
                        run_condition: RunCondition::Playing,
                        behavior_at_zero: BehaviorAtZero::Overflow,
                    };
                }
            } else if self.state == State::Ready {
                c.game.state = self.state;
                c.game.set_play = SetPlay::NoSetPlay;
                c.game.secondary_timer = Timer::Started {
                    remaining: SignedDuration::new(45, 0),
                    run_condition: RunCondition::Always,
                    behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::HlStateShifter(
                        HlStateShifter { state: State::Set },
                    )]),
                };
            } else if self.state == State::Set {
                c.game.state = self.state;
                c.game.set_play = SetPlay::NoSetPlay;
                c.game.secondary_timer = Timer::Stopped;
            } else if self.state == State::Playing {
                c.game.state = self.state;
                c.game.set_play = SetPlay::NoSetPlay;
                c.game.secondary_timer = Timer::Started {
                    remaining: SignedDuration::new(10, 0),
                    run_condition: RunCondition::Always,
                    behavior_at_zero: BehaviorAtZero::Expire(vec![]),
                };
                c.game.set_play = SetPlay::NoSetPlay;
            } else if self.state == State::Finished {
                c.game.state = self.state;
                c.game.set_play = SetPlay::NoSetPlay;
                if c.game.phase != Phase::SecondExtraHalf {
                    c.game.secondary_timer = Timer::Started {
                        remaining: c
                            .params
                            .competition
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
                } else {
                    c.game.primary_timer = Timer::Stopped;
                    c.game.secondary_timer = Timer::Stopped;
                }
                c.game.sec_state_phase = 0;
            } else {
                c.game.state = State::Initial;
            }
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        if c.game.phase == Phase::PenaltyShootout {
            match self.state {
                State::Playing => {
                    c.game.state == State::Set
                        && c.game.teams.values().all(|team| {
                            team.players
                                .iter()
                                .filter(|player| player.penalty != Penalty::Substitute)
                                .count()
                                == 1
                        })
                }
                State::Set => {
                    c.game.state == State::Initial
                        || c.game.state == State::Timeout
                        || c.game.state == State::Finished
                }
                State::Finished => c.game.state == State::Playing,
                _ => false,
            }
        } else {
            match self.state {
                State::Initial => c.game.state == State::Finished,
                State::Ready => c.game.state == State::Initial || c.game.state == State::Timeout,
                State::Set => c.game.state == State::Ready,
                State::Playing => c.game.state == State::Set,
                State::Finished => c.game.state == State::Playing,
                _ => false,
            }
        }
    }
}
