use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::actions::{HlStateShifter, StartSetPlay};
use crate::types::{League, Phase, SetPlay, Side, State};

/// This struct defines an action which corresponds to the referee call "Global Game Stuck".
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GlobalGameStuck;

impl Action for GlobalGameStuck {
    fn execute(&self, c: &mut ActionContext) {
        if c.params.competition.league == League::Spl {
            StartSetPlay {
                side: None,
                set_play: SetPlay::KickOff,
            }
            .execute(c);
        } else {
            c.game.kicking_side = None;
            HlStateShifter {
                state: State::Ready,
            }
            .execute(c);
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase != Phase::PenaltyShootout
            && c.game.state == State::Playing
            && c.params.competition.challenge_mode.is_none()
    }
}
