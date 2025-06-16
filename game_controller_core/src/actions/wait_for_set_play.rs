use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{SetPlay, State};

/// This struct defines an action which corresponds to the referee call "Set". It is the second
/// part of "complex" set plays which have a Ready and Set state.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WaitForSetPlay;

impl Action for WaitForSetPlay {
    fn execute(&self, c: &mut ActionContext) {
        c.game.secondary_timer = Timer::Stopped;
        c.game.state = State::Set;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Ready && c.game.set_play != SetPlay::NoSetPlay
    }
}
