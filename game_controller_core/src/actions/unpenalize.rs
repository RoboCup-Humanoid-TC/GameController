use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{Penalty, PlayerNumber, Side, State};

/// This struct defines an action to unpenalize players.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unpenalize {
    /// The side whose player is unpenalized.
    pub side: Side,
    /// The number of the player who is unpenalized.
    pub player: PlayerNumber,
}

impl Action for Unpenalize {
    fn execute(&self, c: &mut ActionContext) {
        c.game.teams[self.side][self.player].penalty_timer = Timer::Stopped;
        c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.teams[self.side][self.player].penalty != Penalty::NoPenalty
            && c.game.teams[self.side][self.player].penalty != Penalty::Substitute
            && (c.game.teams[self.side][self.player]
                .penalty_timer
                .get_remaining()
                .is_zero()
                // We allow motion in Set penalties to be revoked while still in Set.
                || (c.game.teams[self.side][self.player].penalty == Penalty::MotionInSet
                    && c.game.state == State::Set))
    }
}
