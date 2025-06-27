use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::actions::StartSetPlay;
use crate::types::{Phase, SetPlay, State};

/// This struct defines an action which corresponds to the referee call "Global Game Stuck".
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GlobalGameStuck;

impl Action for GlobalGameStuck {
    fn execute(&self, c: &mut ActionContext) {
        StartSetPlay {
            side: None,
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
