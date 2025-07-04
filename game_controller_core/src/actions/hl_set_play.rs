use crate::action::{Action, ActionContext};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{SecState, Side, State, SetPlay,};
use serde::{Deserialize, Serialize};
pub use time::Duration;

/// This struct defines an action to cycle through the states of a Humanoid League set play.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlSetPlay {
    pub side: Side,
    pub set_play: SecState,
    pub seconds: i64,
}

impl Action for HlSetPlay {
    fn execute(&self, c: &mut ActionContext) {
        let test = SecState::sec_state_to_set_play(self.set_play);
        if c.game.sec_state.state == SecState::Normal {
            c.game.sec_state.state = self.set_play;
            c.game.sec_state.side = self.side;
        } else if c.game.sec_state.state == self.set_play && c.game.sec_state.phase == 0 {
            c.game.sec_state.phase = 1;
            c.game.secondary_timer = Timer::Started {
                remaining: c.params.competition.set_plays[test]
                    .ready_duration
                    .try_into()
                    .unwrap(),
                run_condition: RunCondition::Always,
                behavior_at_zero: BehaviorAtZero::Clip,
            };
        } else if c.game.sec_state.state == self.set_play && c.game.sec_state.phase == 1 {
            c.game.sec_state.phase = 2;
            c.game.secondary_timer = Timer::Stopped;
        } else if c.game.sec_state.state == self.set_play && c.game.sec_state.phase == 2 {
            c.game.sec_state.state = SecState::Normal;
            c.game.sec_state.side = Side::Away;
            c.game.sec_state.phase = 0;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing
            && (c.game.sec_state.state == SecState::Normal
                || (c.game.sec_state.state == self.set_play && c.game.sec_state.side == self.side))
    }
}
