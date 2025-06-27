use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Penalty, Phase, Side, State};

/// This struct defines an action which corresponds to the referee call "Set" in a penalty
/// shoot-out.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WaitForPenaltyShot;

impl Action for WaitForPenaltyShot {
    fn execute(&self, c: &mut ActionContext) {
        // If we come from a previous shot, all players are reset to be substitutes and the sides
        // are switched.
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

        c.game.state = State::Set;
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
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase == Phase::PenaltyShootout
            && (c.game.state == State::Initial
                || c.game.state == State::Timeout
                || (c.game.state == State::Finished
                    && c.game.kicking_side.is_some_and(|side| {
                        ({
                            // At this point, side is the team that just finished its shot, so
                            // -side is the team that would have the next shot. The following
                            // should answer the question: Should that team still have a shot or
                            // not?
                            let score_difference = (c.game.teams[side].score as i16)
                                - (c.game.teams[-side].score as i16);
                            if c.game.teams[-side].penalty_shot < c.params.competition.penalty_shots
                            {
                                // We are still in the regular penalty shoot-out. The following
                                // should answer if still both teams can win.

                                // How many shots does the next team still have in the regular
                                // shoot-out? (is at least 1)
                                let remaining_for_next = c.params.competition.penalty_shots
                                    - c.game.teams[-side].penalty_shot;

                                // How many shots does the last team still have? (can be 0)
                                let remaining_for_last = c.params.competition.penalty_shots
                                    - c.game.teams[side].penalty_shot;

                                // Can the next team still equalize?
                                score_difference <= remaining_for_next.into()
                                    // Can the last team still equalize?
                                    && -score_difference <= remaining_for_last.into()
                            } else if c.game.teams[-side].penalty_shot
                                < c.params.competition.penalty_shots
                                    + c.params.competition.sudden_death_penalty_shots
                            {
                                // This means that the next shot will/would be part of the sudden
                                // death penalty shoot-out. The away team always gets another shot,
                                // but the home team only continues if the score is still even. At
                                // this point, there are other criteria to finish the game even if
                                // neither team scored, but that must be sorted out by the referee.
                                side == Side::Home || score_difference == 0
                            } else {
                                false
                            }
                        } || false)
                    })))
    }
}
