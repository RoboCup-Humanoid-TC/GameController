use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{PlayerNumber, Side};

/// This struct defines an action to apply a penalty to players.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncrementPlayerWarning{
    /// The side whose player is warned.
    pub side: Side,
    /// The number of the player who is warned.
    pub player: PlayerNumber,
}

impl Action for IncrementPlayerWarning {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.teams[self.side][self.player].red < 1 && c.game.teams[self.side][self.player].warnings < 1 {
            c.game.teams[self.side][self.player].warnings += 1;
        } else if c.game.teams[self.side][self.player].warnings == 1 && c.game.teams[self.side][self.player].red < 1 && c.game.teams[self.side][self.player].yellow < 1 {
            c.game.teams[self.side][self.player].yellow += 1;
            c.game.teams[self.side][self.player].warnings = 0;
        } else if c.game.teams[self.side][self.player].red < 1 && c.game.teams[self.side][self.player].yellow == 1 && c.game.teams[self.side][self.player].warnings == 1 {
            c.game.teams[self.side][self.player].red += 1;
            c.game.teams[self.side][self.player].yellow = 0;
            c.game.teams[self.side][self.player].warnings = 0;
        }

    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.teams[self.side][self.player].red < 1
    }
}
