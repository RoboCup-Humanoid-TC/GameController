use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext, VAction};
use crate::actions::FinishSetPlay;
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{SetPlay, State};

/// This struct defines an action which corresponds to the referee call "Playing". It is the third
/// part of "complex" set plays which have a Ready and Set state.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FreeSetPlay;

impl Action for FreeSetPlay {
    fn execute(&self, c: &mut ActionContext) {
        // FinishSetPlay is not a reason to cancel the delayed state because that would mean that
        // e.g. a kick-off is delayed for only 10 seconds instead of the desired 15 seconds.
        if !c.fork(c.params.competition.delay_after_playing, |action| {
            matches!(action, VAction::FinishSetPlay(_))
        }) {
            return;
        }

        if !c.params.competition.set_plays[c.game.set_play]
            .duration
            .is_zero()
        {
            c.game.secondary_timer = Timer::Started {
                remaining: c.params.competition.set_plays[c.game.set_play]
                    .duration
                    .try_into()
                    .unwrap(),
                run_condition: RunCondition::Always,
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::FinishSetPlay(
                    FinishSetPlay,
                )]),
            };
        } else {
            c.game.secondary_timer = Timer::Stopped;
            c.game.set_play = SetPlay::NoSetPlay;
        }
        c.game.timeout_rewind_timer = Timer::Stopped;
        c.game.state = State::Playing;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Set && c.game.set_play != SetPlay::NoSetPlay
    }
}
