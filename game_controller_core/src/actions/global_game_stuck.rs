use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::actions::StartSetPlay;
use crate::types::{Phase, SetPlay, Side, State};

/// This struct defines an action which corresponds to the referee call "Global Game Stuck".
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalGameStuck {
    /// The side which caused the global game stuck. A kick-off is awarded to the other team.
    pub side: Side,
}

impl Action for GlobalGameStuck {
    fn execute(&self, c: &mut ActionContext) {
        StartSetPlay {
            side: -self.side,
            set_play: SetPlay::KickOff,
        }
        .execute(c);
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase != Phase::PenaltyShootout
            && c.game.state == State::Playing
            && c.params.competition.challenge_mode.is_none()
    }
}
