use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{HlCard, PlayerNumber, Side};

/// This struct defines an action to add a card to a player.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlSetGoalkeeper {
    /// The side whose player is shown a card.
    pub side: Side,
    /// The number of the player who is shown a card.
    pub player: PlayerNumber,
}

impl Action for HlSetGoalkeeper {
    fn execute(&self, c: &mut ActionContext) {
        c.game.teams[self.side].goalkeeper = Some(self.player);
        println!(
            "DEBUG: Goalkeeper: {:?}, Playernumber: {:?}\nSide: {:?}",
            c.game.teams[self.side].goalkeeper, self.player, self.side
        );
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.teams[self.side].goalkeeper != Some(self.player)
    }
}
