use serde::{Deserialize, Serialize};
pub use time::Duration;
use crate::action::{Action, ActionContext, VAction};
use crate::actions::{HlUnpenalize};
use crate::timer::{BehaviorAtZero, RunCondition, SignedDuration, Timer};
use crate::types::{Penalty, PlayerNumber, Side, State};

/// This struct defines an action to substitute players.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlBallManipulation {
    /// The side which does the substitution.
    pub side: Side,
    /// The player who will be substituted.
    pub player: PlayerNumber,
}

impl Action for HlBallManipulation {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.teams[self.side][self.player].penalty == Penalty::NoPenalty
        {
            c.game.teams[self.side][self.player].penalty = Penalty::BallHolding;
            c.game.teams[self.side][self.player].penalty_timer = Timer::Started {
                remaining: c.params.competition.penalties[Penalty::BallHolding].duration.try_into().unwrap(),
                run_condition: RunCondition::Playing,
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::HlUnpenalize(HlUnpenalize {
                    side: self.side,
                    player: self.player,
                    timer: false,
                })]),
            };
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.teams[self.side][self.player].penalty == Penalty::NoPenalty
    }
}
