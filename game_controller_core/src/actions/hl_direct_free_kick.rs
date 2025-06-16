use serde::{Deserialize, Serialize};
pub use time::Duration;
use crate::action::{Action, ActionContext};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Side, State, SecState};

/// This struct defines an action for when a goal has been scored.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlDirectFreeKick{
    pub side: Side,
    pub seconds: i64,
}

impl Action for HlDirectFreeKick {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.sec_state.state == SecState::Normal {
            c.game.sec_state.state = SecState::DirectFreekick;
            c.game.sec_state.side = self.side;
        } else if c.game.sec_state.state == SecState::DirectFreekick && c.game.sec_state.phase == 0 {
            c.game.sec_state.phase = 1;
            c.game.secondary_timer = Timer::Started {
                remaining: Duration::new(self.seconds,0),
                run_condition: RunCondition::Always,
                behavior_at_zero: BehaviorAtZero::Clip,
            };
        } else if c.game.sec_state.state == SecState::DirectFreekick && c.game.sec_state.phase == 1 {
            c.game.sec_state.phase = 2;
            c.game.secondary_timer = Timer::Stopped;
        } else if c.game.sec_state.state == SecState::DirectFreekick && c.game.sec_state.phase == 2 {
            c.game.sec_state.state = SecState::Normal;
            c.game.sec_state.side = Side::Away;
            c.game.sec_state.phase = 0;
        }

    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing && (c.game.sec_state.state == SecState::Normal || (c.game.sec_state.state == SecState::DirectFreekick
            && c.game.sec_state.side == self.side))
    }
}