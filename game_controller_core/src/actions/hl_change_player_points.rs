use std::mem::replace;

use enum_map::enum_map;
use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{Penalty, Phase, Player, PlayerNumber, Side};

/// This struct defines an action to select the player in a penalty shoot-out.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlChangePlayerPoints {
    /// The side which selects a player.
    pub side: Side,
    /// The player who is selected.
    pub player: PlayerNumber,
    /// Whether the player is a goalkeeper (i.e. wearing a goalkeeper jersey).
    pub increase: bool,
}

impl Action for HlChangePlayerPoints {
    fn execute(&self, c: &mut ActionContext) {
        if self.increase {
            c.game.teams[self.side][self.player].points += 1;
        } 
        else 
        {
            c.game.teams[self.side][self.player].points -= 1;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        if c.params.competition.name == "Drop In"
        {
            true
        }
        else
        {
            false
        }
    }
}
