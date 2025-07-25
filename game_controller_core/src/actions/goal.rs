use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext, VAction};
use crate::actions::{HlStateShifter, StartSetPlay};
use crate::timer::{BehaviorAtZero, RunCondition, SignedDuration, Timer};
use crate::types::{League, Penalty, Phase, SetPlay, Side, State};

/// This struct defines an action for when a goal has been scored.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    /// The side which has scored a goal.
    pub side: Side,
}

impl Action for Goal {
    fn execute(&self, c: &mut ActionContext) {
        if c.params.competition.league == League::Spl {
            // Mercy rule: At a certain goal difference, the game is finished.
            let mercy_rule = c.game.phase != Phase::PenaltyShootout
                && !c.game.teams[self.side].illegal_communication
                && (c.game.teams[self.side].score + 1)
                    >= c.game.teams[-self.side].score
                        + c.params.competition.mercy_rule_score_difference;
            if c.game.phase != Phase::PenaltyShootout
                && !mercy_rule
                && !c.fork(c.params.competition.delay_after_goal, |_| false)
            {
                return;
            }

            c.game.secondary_timer = Timer::Stopped;
            c.game.set_play = SetPlay::NoSetPlay;

            if !c.game.teams[self.side].illegal_communication {
                c.game.teams[self.side].score += 1;
            }
            if mercy_rule {
                c.game.teams.values_mut().for_each(|team| {
                    team.players.iter_mut().for_each(|player| {
                        player.penalty_timer = Timer::Stopped;
                    })
                });
                c.game.phase = Phase::SecondHalf;
                c.game.state = State::Finished;
            } else if c.game.phase != Phase::PenaltyShootout {
                // A kick-off for the other team.
                StartSetPlay {
                    side: Some(-self.side),
                    set_play: SetPlay::KickOff,
                }
                .execute(c);
            } else {
                c.game.teams[self.side].penalty_shot_mask |=
                    1u16 << (c.game.teams[self.side].penalty_shot - 1);
                c.game.state = State::Finished;
            }
        } else if c.params.competition.league == League::Humanoid {
            c.game.teams[self.side].score += 1;
            if c.game.phase != Phase::PenaltyShootout {
                c.game.kicking_side = Some(-self.side);
                c.game.secondary_timer = Timer::Started {
                    remaining: SignedDuration::new(45, 0),
                    run_condition: RunCondition::Always,
                    behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::HlStateShifter(
                        HlStateShifter { state: State::Set },
                    )]),
                };
                c.game.state = State::Ready;
                if c.params.competition.name == "Drop In" {
                    c.game.teams[self.side]
                        .players
                        .iter_mut()
                        .for_each(|player| {
                            if player.penalty == Penalty::NoPenalty {
                                player.points += 1;
                            }
                        });
                    c.game.teams[-self.side]
                        .players
                        .iter_mut()
                        .for_each(|player| {
                            if player.penalty == Penalty::NoPenalty {
                                player.points -= 1;
                            } else if player.penalty != Penalty::Substitute {
                                player.points -= 2;
                            }
                        });
                }
            } else {
                c.game.state = State::Finished;
            }
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        if c.params.competition.league == League::Spl {
            c.game.state == State::Playing
                && (c.game.phase != Phase::PenaltyShootout
                    || c.game.kicking_side.is_none_or(|side| side == self.side))
                && (c.params.competition.challenge_mode.is_none() || self.side == Side::Home)
        } else if c.params.competition.league == League::Humanoid {
            c.game.state == State::Playing
                && (c.game.phase != Phase::PenaltyShootout
                    || c.game.kicking_side.is_none_or(|side| side == self.side))
        } else {
            false
        }
    }
}
