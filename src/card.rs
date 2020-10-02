// The cards used by the poker game
use super::value::Value;
use crate::suit::Suit;

use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}
impl Card {
    pub fn get_deck() -> HashSet<Card> {
        let mut deck: HashSet<Card> = HashSet::new();
        for suit in ["Hearts", "Diamonds", "Spades", "Clubs"].iter() {
            for value in 1..14 {
                deck.insert(Card {
                    suit: Suit::new(suit),
                    value: Value::new(value),
                });
            }
        }
        deck
    }
    pub fn is_suit(&self, suit: Suit) -> bool {
        if self.suit == suit {
            true
        } else {
            false
        }
    }
}
