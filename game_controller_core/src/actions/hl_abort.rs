use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{SetPlay, Side, State};
use serde::{Deserialize, Serialize};

/// This struct defines an action for when a goal has been scored.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlAbort {
    pub side: Side,
}

impl Action for HlAbort {
    fn execute(&self, c: &mut ActionContext) {
        c.game.set_play = SetPlay::NoSetPlay;
        c.game.kicking_side = None;
        c.game.sec_state_phase = 0;
        c.game.secondary_timer = Timer::Stopped;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing
            && c.game.set_play != SetPlay::NoSetPlay
            && c.game.kicking_side.is_some_and(|side| side == self.side)
    }
}
