use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{Penalty, State, PlayerNumber, Side};

/// This struct defines an action to substitute players.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlSubstitute {
    /// The side which does the substitution.
    pub side: Side,
    /// The player who will be substituted.
    pub player: PlayerNumber,
}

impl Action for HlSubstitute {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.teams[self.side][self.player].penalty == Penalty::NoPenalty
        {
            c.game.teams[self.side][self.player].penalty = Penalty::Substitute;
        } else {
            let mut unsubs = 0;
            for idx in 1..20 {
                if c.game.teams[self.side][PlayerNumber::new(idx)].penalty != Penalty::Substitute {
                    unsubs += 1;
                }
            }
            if unsubs < 5 {
                c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
            }
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        true
    }
}
