use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{SecState, Side, State};
use serde::{Deserialize, Serialize};

/// This struct defines an action for when a goal has been scored.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlAbort {
    pub side: Side,
}

impl Action for HlAbort {
    fn execute(&self, c: &mut ActionContext) {
        c.game.sec_state.state = SecState::Normal;
        c.game.sec_state.side = Side::Away;
        c.game.sec_state.phase = 0;
        c.game.secondary_timer = Timer::Stopped;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing
            && c.game.sec_state.state != SecState::Normal
            && c.game.sec_state.side == self.side
    }
}
