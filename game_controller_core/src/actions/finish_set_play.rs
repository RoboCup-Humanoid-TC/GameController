use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{SetPlay, State};

/// This struct defines an action which corresponds to the referee call "Ball Free". It is the last
/// part of a set play (i.e. fourth part of "complex" set plays with Ready and Set state and second
/// part of "simple" set plays).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FinishSetPlay;

impl Action for FinishSetPlay {
    fn execute(&self, c: &mut ActionContext) {
        c.game.secondary_timer = Timer::Stopped;
        c.game.set_play = SetPlay::NoSetPlay;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing && c.game.set_play != SetPlay::NoSetPlay
    }
}
