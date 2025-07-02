use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{Phase, State};

/// This struct defines an action which corresponds to the referee call "Finish" in a penalty
/// shoot-out.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FinishPenaltyShot;

impl Action for FinishPenaltyShot {
    fn execute(&self, c: &mut ActionContext) {
        // Cancel all penalty timers (only for consistency with FinishHalf).
        c.game.teams.values_mut().for_each(|team| {
            team.players.iter_mut().for_each(|player| {
                player.penalty_timer = Timer::Stopped;
            })
        });

        c.game.state = State::Finished;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase == Phase::PenaltyShootout && c.game.state == State::Playing
    }
}
