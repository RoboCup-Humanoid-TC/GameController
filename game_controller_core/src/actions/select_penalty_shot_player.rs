use std::mem::replace;

use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{Penalty, Phase, Player, PlayerNumber, Side};

/// This struct defines an action to select the player in a penalty shoot-out.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectPenaltyShotPlayer {
    /// The side which selects a player.
    pub side: Side,
    /// The player who is selected.
    pub player: PlayerNumber,
    /// Whether the player is a goalkeeper (i.e. wearing a goalkeeper jersey).
    pub goalkeeper: bool,
}

impl Action for SelectPenaltyShotPlayer {
    fn execute(&self, c: &mut ActionContext) {
        // Penalize all players while transferring the penalty of the previously selected player to
        // the new player. If no player was previously selected, it has no penalty.
        c.game.teams[self.side][self.player] = c.game.teams[self.side]
            .players
            .iter_mut()
            .map(|player| {
                replace(
                    player,
                    Player {
                        penalty: Penalty::Substitute,
                        penalty_timer: Timer::Stopped,
                        warnings: 0,
                        yellow: 0,
                        red: 0,
                        goalkeeper: 0,
                    },
                )
            })
            .find(|player| player.penalty != Penalty::Substitute)
            .unwrap_or(Player {
                penalty: Penalty::NoPenalty,
                penalty_timer: Timer::Stopped,
                warnings: 0,
                yellow: 0,
                red: 0,
                goalkeeper: 0,
            });

        c.game.teams[self.side].goalkeeper = if self.goalkeeper {
            Some(self.player)
        } else {
            None
        };
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase == Phase::PenaltyShootout
    }
}
