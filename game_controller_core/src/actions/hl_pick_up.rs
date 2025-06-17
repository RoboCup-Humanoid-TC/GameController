use crate::action::{Action, ActionContext};
use crate::types::{Penalty, PlayerNumber, Side, State};
use serde::{Deserialize, Serialize};

/// This struct defines an action to substitute players.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlPickUp {
    /// The side which does the substitution.
    pub side: Side,
    /// The player who will be substituted.
    pub player: PlayerNumber,
}

impl Action for HlPickUp {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.teams[self.side][self.player].penalty == Penalty::NoPenalty {
            c.game.teams[self.side][self.player].penalty = Penalty::PickedUp;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.teams[self.side][self.player].penalty == Penalty::NoPenalty
    }
}
