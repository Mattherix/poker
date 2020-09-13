// The cards used by the poker game
use self::Suit::*;
use std::slice::Iter;

#[derive(PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}
impl Suit {
    pub fn iter() -> Iter<'static, Suit> {
        static SUIT: [Suit; 4] = [Hearts, Diamonds, Spades, Clubs];
        SUIT.iter()
    }
}

pub struct Value(u8);
impl Value {
    pub fn new(number: u8) -> Value {
        if number < 1 || number > 13 {
            panic!("Card value must be between 1 and 13");
        }
        Value(number)
    }
}

pub struct Card {
    pub suit: Suit,
    pub value: Value
}
impl Card {
    pub fn new(suit: Suit, value: Value) -> Card {
        Card {
            suit: suit,
            value: value
        } 
    }
    pub fn get_deck() -> Vec<Card> {
        for elem in iter {
            
        }
    }
    pub fn is_suit(&self, suit: Suit) -> bool {
        if self.suit == suit {
            true
        } else {
            false
        }
    }
}