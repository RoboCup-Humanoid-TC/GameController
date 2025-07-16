use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{Phase, Side, State};

/// This struct defines an action that sets the kicking side before a penalty shoot-out.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlSetKickingSide {
    /// The side which should have the first kick.
    pub side: Side,
}

impl Action for HlSetKickingSide {
    fn execute(&self, c: &mut ActionContext) {
        c.game.kicking_side = Some(self.side);
        c.game.sides = -c.game.sides;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase == Phase::PenaltyShootout
            && c.game.state == State::Initial
            && c.game.kicking_side.is_none_or(|side| side != self.side)
    }
}
