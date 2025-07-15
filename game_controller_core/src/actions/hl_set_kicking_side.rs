use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{Side, State, SecState};

/// This struct defines an action to add a card to a player.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlSetKickingSide {
    /// The side whose should have kickoff.
    pub side: Side,
}

impl Action for HlSetKickingSide {
    fn execute(&self, c: &mut ActionContext) {
        c.game.kicking_side = Some(self.side);
        c.game.sides = -c.game.sides;
        println!("DEBUG: Kicking side set to: {:?}", self.side);
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.kicking_side != Some(self.side) &&
        c.game.state == State::Initial &&
        c.game.sec_state.state == SecState::Penaltyshoot
    }
}
