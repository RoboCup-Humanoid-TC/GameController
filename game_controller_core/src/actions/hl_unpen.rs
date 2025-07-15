use crate::action::{Action, ActionContext, VAction};
use crate::actions::HlNormalize;
use crate::timer::{BehaviorAtZero, EvaluatedRunConditions, RunCondition, Timer};
use crate::types::{HlCard, Penalty, PlayerNumber, SecState, Side, State};
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
        if c.game.teams[self.side][self.player].penalty != Penalty::NoPenalty
            && c.game.teams[self.side][self.player].penalty != Penalty::Substitute
            && c.game.teams[self.side][self.player].penalty_timer == Timer::Stopped
        {
            c.game.teams[self.side][self.player].penalty_timer = Timer::Started {
                remaining: c.params.competition.penalties
                    [c.game.teams[self.side][self.player].penalty]
                    .duration
                    .try_into()
                    .unwrap(),
                run_condition: RunCondition::Playing,
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::HlNormalize(HlNormalize {
                    side: self.side,
                    player: self.player,
                })]),
            };
        } else if c.game.teams[self.side][self.player].penalty != Penalty::NoPenalty
            && c.game.teams[self.side][self.player].penalty != Penalty::Substitute
            && c.game.teams[self.side][self.player].penalty_timer != Timer::Stopped
        {
            c.game.teams[self.side][self.player].penalty_timer = Timer::Stopped;
        } else if c.game.teams[self.side][self.player].penalty == Penalty::Substitute {
            if c.game.sec_state.state != SecState::Penaltyshoot {
                let unsubs = c.game.teams[self.side]
                    .players
                    .iter()
                    .filter(|player| player.penalty != Penalty::Substitute)
                    .count();
                if unsubs < c.params.competition.players_per_team.into() {
                    c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
                }
            } else {
                let unsubs = c.game.teams[self.side]
                    .players
                    .iter()
                    .filter(|player| player.penalty != Penalty::Substitute)
                    .count();
                if unsubs < 1 {
                    c.game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
                }
            }
        } else {
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        if c.game.teams[self.side][self.player].penalty == Penalty::Substitute {
            let unsubs = c.game.teams[self.side]
                .players
                .iter()
                .filter(|player| player.penalty != Penalty::Substitute)
                .count();
            if c.game.sec_state.state == SecState::Penaltyshoot {
                c.game.state != State::Initial
                    && c.game.teams[self.side][self.player].cards[HlCard::Red] < 1
                    && unsubs < 1
            } else {
                unsubs < c.params.competition.players_per_team.into()
            }
        } else {
            c.game.teams[self.side][self.player].penalty != Penalty::NoPenalty &&
                // It isn't possible to unpenalize the goalkeeper in a penalty shoot-out.
                !(c.game.sec_state.state == SecState::Penaltyshoot && c.game.state == State::Playing && c.game.kicking_side.is_none_or(|side| side != self.side))
        }
    }
}
