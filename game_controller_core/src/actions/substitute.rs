use std::mem::replace;

use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Penalty, Phase, PlayerNumber, Side, State};

/// This struct defines an action to substitute players.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Substitute {
    /// The side which does the substitution.
    pub side: Side,
    /// The player who comes in (currently a substitute).
    pub player_in: PlayerNumber,
    /// The player who comes off (will become a substitute).
    pub player_out: PlayerNumber,
}

impl Action for Substitute {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.teams[self.side][self.player_out].penalty == Penalty::NoPenalty
            && matches!(c.game.state, State::Ready | State::Set | State::Playing)
        {
            // Players that are substituted while not being penalized must still wait as if they
            // were picked-up.
            assert!(!c.params.competition.penalties[Penalty::PickedUp].incremental);
            c.game.teams[self.side][self.player_in].penalty = Penalty::PickedUp;
            c.game.teams[self.side][self.player_in].penalty_timer = Timer::Started {
                remaining: c.params.competition.penalties[Penalty::PickedUp]
                    .duration
                    .try_into()
                    .unwrap(),
                run_condition: RunCondition::ReadyOrPlaying,
                behavior_at_zero: BehaviorAtZero::Clip,
            };
            c.game.teams[self.side][self.player_out].penalty_timer = Timer::Stopped;
        } else {
            // Inherit the penalty and the timer.
            c.game.teams[self.side][self.player_in].penalty =
                c.game.teams[self.side][self.player_out].penalty;
            c.game.teams[self.side][self.player_in].penalty_timer = replace(
                &mut c.game.teams[self.side][self.player_out].penalty_timer,
                Timer::Stopped,
            );
        }
        c.game.teams[self.side][self.player_out].penalty = Penalty::Substitute;
        if c.game.teams[self.side].goalkeeper == Some(self.player_out) {
            c.game.teams[self.side].goalkeeper = Some(self.player_in);
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase != Phase::PenaltyShootout
            && self.player_in != self.player_out
            && c.game.teams[self.side][self.player_in].penalty == Penalty::Substitute
            && c.game.teams[self.side][self.player_out].penalty != Penalty::Substitute
            && c.params.competition.challenge_mode.is_none()
    }
}
