use crate::action::{Action, ActionContext};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Phase, SetPlay, Side, State};
use serde::{Deserialize, Serialize};
pub use time::Duration;

/// This struct defines an action to cycle through the states of a Humanoid League set play.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlSetPlay {
    pub side: Side,
    pub set_play: SetPlay,
}

impl Action for HlSetPlay {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.set_play == SetPlay::NoSetPlay {
            c.game.set_play = self.set_play;
            c.game.kicking_side = Some(self.side);
        } else if c.game.set_play == self.set_play {
            match c.game.sec_state_phase {
                0 => {
                    c.game.sec_state_phase = 1;
                    c.game.secondary_timer = Timer::Started {
                        remaining: c.params.competition.set_plays[self.set_play]
                            .ready_duration
                            .try_into()
                            .unwrap(),
                        run_condition: RunCondition::Always,
                        behavior_at_zero: BehaviorAtZero::Clip,
                    };
                }
                1 => {
                    c.game.sec_state_phase = 2;
                    c.game.secondary_timer = Timer::Stopped;
                }
                2 => {
                    c.game.set_play = SetPlay::NoSetPlay;
                    c.game.sec_state_phase = 0;
                }
                _ => {}
            };
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase != Phase::PenaltyShootout
            && c.game.state == State::Playing
            && (c.game.set_play == SetPlay::NoSetPlay
                || (c.game.set_play == self.set_play
                    && c.game.kicking_side.is_some_and(|side| side == self.side)))
    }
}
