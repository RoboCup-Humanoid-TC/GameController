use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{Penalty, Phase, State};

/// This struct defines an action which corresponds to the referee call "Playing" in a penalty
/// shoot-out.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FreePenaltyShot;

impl Action for FreePenaltyShot {
    fn execute(&self, c: &mut ActionContext) {
        if !c.fork(c.params.competition.delay_after_playing, |_| false) {
            return;
        }

        c.game.state = State::Playing;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase == Phase::PenaltyShootout
            && c.game.state == State::Set
            && c.game.teams.values().all(|team| {
                team.players
                    .iter()
                    .filter(|player| player.penalty != Penalty::Substitute)
                    .count()
                    == 1
            })
    }
}
