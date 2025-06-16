use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext, VAction};
use crate::actions::{FinishSetPlay, WaitForSetPlay};
use crate::timer::{BehaviorAtZero, RunCondition, SignedDuration, Timer};
use crate::types::{Phase, SetPlay, Side, State};

/// This struct defines an action to start a set play. Depending on the set play type, this means
/// switching to the Ready state or just setting a flag for the current set play within the Playing
/// state.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartSetPlay {
    /// The side which can execute the set play.
    pub side: Side,
    /// The type of set play to start.
    pub set_play: SetPlay,
}

impl Action for StartSetPlay {
    fn execute(&self, c: &mut ActionContext) {
        // Cancel all penalty timers if starting a set play from any state other than Playing.
        if c.game.state != State::Playing {
            c.game.teams.values_mut().for_each(|team| {
                team.players.iter_mut().for_each(|player| {
                    player.penalty_timer = Timer::Stopped;
                })
            });
        }

        if !c.params.competition.set_plays[self.set_play]
            .ready_duration
            .is_zero()
        {
            c.game.secondary_timer = Timer::Started {
                remaining: c.params.competition.set_plays[self.set_play]
                    .ready_duration
                    .try_into()
                    .unwrap(),
                run_condition: RunCondition::Always,
                // Automatically transition to the Set state when the timer expires.
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::WaitForSetPlay(
                    WaitForSetPlay,
                )]),
            };
            // This timer counts the time during the Ready and Set states (negatively) so it can be
            // added back to the primary timer when taking a timeout. It uses the same run
            // condition as the primary timer, so if the primary counter doesn't count down, the
            // time won't be added back to it.
            c.game.timeout_rewind_timer = Timer::Started {
                remaining: SignedDuration::ZERO,
                run_condition: RunCondition::Playing,
                behavior_at_zero: BehaviorAtZero::Overflow,
            };
            c.game.state = State::Ready;
        } else {
            c.game.secondary_timer = Timer::Started {
                remaining: c.params.competition.set_plays[self.set_play]
                    .duration
                    .try_into()
                    .unwrap(),
                run_condition: RunCondition::Always,
                // Automatically deactivate the set play when the timer expires.
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::FinishSetPlay(
                    FinishSetPlay,
                )]),
            };
        }
        c.game.set_play = self.set_play;
        c.game.kicking_side = self.side;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        self.set_play != SetPlay::NoSetPlay
            && c.game.phase != Phase::PenaltyShootout
            && (if self.set_play == SetPlay::KickOff {
                // For kick-offs, the kicking side is pre-filled so that only that team can take
                // the kick-off.
                (c.game.state == State::Initial || c.game.state == State::Timeout)
                    && c.game.kicking_side == self.side
            } else {
                // It must be Playing, and we can only start set play during other set plays if
                // they are for the other team (this is a shortcut, because FinishSetPlay should
                // have been clicked before).
                c.game.state == State::Playing
                    && (c.game.set_play == SetPlay::NoSetPlay || c.game.kicking_side != self.side)
                    && c.params.competition.challenge_mode.is_none()
            })
    }
}
