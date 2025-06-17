use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::State;

/// This struct defines an action that adds a minute of extra time.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddExtraTime;

impl AddExtraTime {
    const MINUTE: Duration = Duration::from_secs(60);
}

impl Action for AddExtraTime {
    fn execute(&self, c: &mut ActionContext) {
        c.game.primary_timer = Timer::Started {
            remaining: c.game.primary_timer.get_remaining() + Self::MINUTE,
            run_condition: RunCondition::Playing,
            behavior_at_zero: BehaviorAtZero::Overflow,
        };
        c.game.teams.values_mut().for_each(|team| {
            if !team.illegal_communication {
                team.message_budget += c.params.competition.messages_per_team_per_extra_minute;
            }
        });
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing
    }
}
