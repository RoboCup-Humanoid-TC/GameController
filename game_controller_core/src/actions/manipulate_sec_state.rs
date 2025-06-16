use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{State, SecState};

/// This struct defines an action for when a goal has been scored.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManipulateSecState;

impl Action for ManipulateSecState {
    fn execute(&self, c: &mut ActionContext) {
        c.game.sec_state.state = SecState::Penalityshoot;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing
    }
}