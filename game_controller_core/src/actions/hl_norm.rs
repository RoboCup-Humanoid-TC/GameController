use crate::action::{Action, ActionContext, VAction};
use crate::timer::{BehaviorAtZero, EvaluatedRunConditions, RunCondition, Timer};
use crate::types::{Penalty, PlayerNumber, Side};
use serde::{Deserialize, Serialize};
pub use time::Duration;

/// This struct defines an action to free players from all substitutions.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlNormalize {
    /// The side which has the substitution.
    pub side: Side,
    /// The player who will be freed.
    pub player: PlayerNumber,
}

impl Action for HlNormalize {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.teams[self.side][self.player].penalty != Penalty::NoPenalty {
            c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
            c.game.teams[self.side][self.player].penalty_timer = Timer::Stopped;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        if c.game.teams[self.side][self.player].penalty != Penalty::NoPenalty {
            true
        } else {
            false
        }
    }
}
