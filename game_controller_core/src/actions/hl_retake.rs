use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{SecState, Side, State};

/// This struct defines an action for when a goal has been scored.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlRetake {
    pub side: Side,
}

impl Action for HlRetake {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.sec_state.phase != 0 {
            c.game.sec_state.phase = 0;
            c.game.secondary_timer = Timer::Stopped;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing
            && c.game.sec_state.phase != 0
            && c.game.sec_state.state != SecState::Normal
            && c.game.sec_state.side == self.side
    }
}
