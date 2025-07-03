use crate::action::{Action, ActionContext, VAction};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Penalty, PlayerNumber, Side};
use serde::{Deserialize, Serialize};
pub use time::Duration;

/// This struct defines an action to substitute players.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlUnpenalize {
    /// The side which does the substitution.
    pub side: Side,
    /// The player who will be substituted.
    pub player: PlayerNumber,
}

impl Action for HlUnpenalize {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.teams[self.side][self.player]
                .penalty_timer
                .get_remaining()
                != Duration::new(0, 0)
        {
            c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
            c.game.teams[self.side][self.player].penalty_timer = Timer::Stopped;
        }
        else if c.game.teams[self.side][self.player]
                .penalty_timer
                .get_remaining()
                == Duration::new(0, 0) && 
                c.game.teams[self.side][self.player].penalty 
                != Penalty::NoPenalty && 
                c.game.teams[self.side][self.player].penalty 
                != Penalty::Substitute
        {
            c.game.teams[self.side][self.player].penalty_timer = Timer::Started {
                        remaining: c.params.competition.penalties[
                            c.game.teams[self.side][self.player].penalty
                            ]
                            .duration
                            .try_into()
                            .unwrap(),
                        run_condition: RunCondition::Playing,
                        behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::HlUnpenalize(
                            HlUnpenalize {
                                side: self.side,
                                player: self.player,
                            },
                        )]),
                    };
        } 
        else if c.game.teams[self.side][self.player].penalty == Penalty::Substitute 
        {
            let unsubs = c.game.teams[self.side]
                .players
                .iter()
                .filter(|player| player.penalty != Penalty::Substitute)
                .count();
            if unsubs < c.params.competition.players_per_team.into() {
                c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
            }
        } 
        else 
        {
            c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
            c.game.teams[self.side][self.player].penalty_timer = Timer::Stopped;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.teams[self.side][self.player].penalty != Penalty::NoPenalty
    }
}
