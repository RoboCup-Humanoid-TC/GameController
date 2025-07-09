use crate::action::{Action, ActionContext, VAction};
use crate::actions::HlUnpenalize;
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Penalty, PlayerNumber, Side};
use serde::{Deserialize, Serialize};

/// This struct defines an action to substitute players.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlPenalize {
    /// The side which does the substitution.
    pub side: Side,
    /// The player who will be substituted.
    pub player: PlayerNumber,
    /// The player who will be substituted.
    pub penalty: Penalty,
}

impl Action for HlPenalize {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.teams[self.side][self.player].penalty == Penalty::NoPenalty {
            c.game.teams[self.side][self.player].penalty = self.penalty;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.teams[self.side][self.player].penalty == Penalty::NoPenalty
    }
}
