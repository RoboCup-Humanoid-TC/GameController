use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{HlCard, Penalty, PlayerNumber, Side};

/// This struct defines an action to substitute players.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlSubstitute {
    /// The side which does the substitution.
    pub side: Side,
    /// The player who will be substituted.
    pub player: PlayerNumber,
}

impl Action for HlSubstitute {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.teams[self.side][self.player].penalty == Penalty::NoPenalty {
            c.game.teams[self.side][self.player].penalty = Penalty::Substitute;
        } else {
            let unsubs = c.game.teams[self.side]
                .players
                .iter()
                .filter(|player| player.penalty != Penalty::Substitute)
                .count();
            if unsubs < c.params.competition.players_per_team.into() {
                c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
            }
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        if c.game.teams[self.side][self.player].cards[HlCard::Red] >= 1 {
            false
        } else {
            true
        }
    }
}
