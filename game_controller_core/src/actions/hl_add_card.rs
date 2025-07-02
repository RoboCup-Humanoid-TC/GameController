use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{HlCard, PlayerNumber, Side};

/// This struct defines an action to add a card to a player.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HlAddCard {
    /// The side whose player is shown a card.
    pub side: Side,
    /// The number of the player who is shown a card.
    pub player: PlayerNumber,
    /// The card that is shown.
    pub card: HlCard,
}

impl Action for HlAddCard {
    fn execute(&self, c: &mut ActionContext) {
        // TODO: This logic can surely be implemented in a much simpler way.
        match self.card {
            HlCard::Warning => {
                if c.game.teams[self.side][self.player].cards[HlCard::Red] < 1
                    && c.game.teams[self.side][self.player].cards[HlCard::Warning] < 1
                {
                    c.game.teams[self.side][self.player].cards[HlCard::Warning] += 1;
                } else if c.game.teams[self.side][self.player].cards[HlCard::Warning] == 1
                    && c.game.teams[self.side][self.player].cards[HlCard::Red] < 1
                    && c.game.teams[self.side][self.player].cards[HlCard::Yellow] < 1
                {
                    c.game.teams[self.side][self.player].cards[HlCard::Yellow] += 1;
                    c.game.teams[self.side][self.player].cards[HlCard::Warning] = 0;
                } else if c.game.teams[self.side][self.player].cards[HlCard::Red] < 1
                    && c.game.teams[self.side][self.player].cards[HlCard::Yellow] == 1
                    && c.game.teams[self.side][self.player].cards[HlCard::Warning] == 1
                {
                    c.game.teams[self.side][self.player].cards[HlCard::Red] += 1;
                    c.game.teams[self.side][self.player].cards[HlCard::Yellow] = 0;
                    c.game.teams[self.side][self.player].cards[HlCard::Warning] = 0;
                }
            }
            HlCard::Yellow => {
                if c.game.teams[self.side][self.player].cards[HlCard::Yellow] < 1
                    && c.game.teams[self.side][self.player].cards[HlCard::Red] < 1
                {
                    c.game.teams[self.side][self.player].cards[HlCard::Yellow] += 1;
                } else if c.game.teams[self.side][self.player].cards[HlCard::Yellow] == 1
                    && c.game.teams[self.side][self.player].cards[HlCard::Red] < 1
                {
                    c.game.teams[self.side][self.player].cards[HlCard::Red] += 1;
                    c.game.teams[self.side][self.player].cards[HlCard::Yellow] = 0;
                    c.game.teams[self.side][self.player].cards[HlCard::Warning] = 0;
                }
            }
            HlCard::Red => {
                if c.game.teams[self.side][self.player].cards[HlCard::Red] < 1 {
                    c.game.teams[self.side][self.player].cards[HlCard::Red] += 1;
                    c.game.teams[self.side][self.player].cards[HlCard::Yellow] = 0;
                    c.game.teams[self.side][self.player].cards[HlCard::Warning] = 0;
                }
            }
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.teams[self.side][self.player].cards[HlCard::Red] < 1
    }
}
