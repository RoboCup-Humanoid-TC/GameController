use serde::{Deserialize, Serialize};
pub use time::Duration;
use crate::action::{Action, ActionContext, VAction};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Penalty, PlayerNumber, Side};

/// This struct defines an action to substitute players.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlUnpenalize {
    /// The side which does the substitution.
    pub side: Side,
    /// The player who will be substituted.
    pub player: PlayerNumber,
    /// defines if a timer should be started or the player is unpenalized after click
    pub timer: bool,
}

impl Action for HlUnpenalize {
    fn execute(&self, c: &mut ActionContext) {
        if self.timer && c.game.teams[self.side][self.player].penalty_timer.get_remaining() != Duration::new(0,0) {
            c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
            c.game.teams[self.side][self.player].penalty_timer = Timer::Stopped;
        } else if self.timer && c.game.teams[self.side][self.player].penalty_timer.get_remaining() == Duration::new(0,0)  {
            c.game.teams[self.side][self.player].penalty_timer = Timer::Started {
                remaining: c.params.competition.penalties[Penalty::PickedUp].duration.try_into().unwrap(),
                run_condition: RunCondition::Playing,
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::HlUnpenalize(HlUnpenalize{
                    side: self.side,
                    player: self.player,
                    timer: false,
                })]),
            };
        } else if c.game.teams[self.side][self.player].penalty == Penalty::Substitute {
            let mut unsubs = 0;
            for idx in 1..20 {
                if c.game.teams[self.side][PlayerNumber::new(idx)].penalty != Penalty::Substitute {
                    unsubs += 1;
                }
            }
            if unsubs < 5 {
                c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
            }
        } else {
            c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
            c.game.teams[self.side][self.player].penalty_timer = Timer::Stopped;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.teams[self.side][self.player].penalty != Penalty::NoPenalty
    }
}
