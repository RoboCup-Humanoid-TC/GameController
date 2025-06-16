use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{State};

/// This struct defines an action to start a set play. Depending on the set play type, this means
/// switching to the Ready state or just setting a flag for the current set play within the Playing
/// state.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Humanoid;

impl Action for Humanoid {
    fn execute(&self, c: &mut ActionContext) {
        c.game.league = "HL"
        
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        
    }
}