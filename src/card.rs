// The cards used by the poker game
use std::collections::HashSet;
use self::Suit::*;

#[derive(Eq, PartialEq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}
impl Suit {
    pub fn new(suit: &str) -> Suit {
        match suit {
            "Hearts" => Suit::Hearts,
            "Diamonds" => Suit::Diamonds,
            "Spades" => Suit::Spades,
            "Clubs" => Suit::Clubs,
            _ => panic!("Card can only be Hearts, Diamonds, Spades or Clubs")
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Value(u8);
impl Value {
    pub fn new(number: u8) -> Value {
        if number < 1 || number > 13 {
            panic!("Card value must be between 1 and 13");
        }
        Value(number)
    }
}

#[derive(Eq, PartialEq, Hash)]
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
    pub fn get_deck() -> HashSet<Card> {
        let mut deck: HashSet<Card> = HashSet::new();
        for suit in ["Hearts", "Diamonds", "Spades", "Clubs"].iter() {
            for value in 1..14 {
                deck.insert(
                    Card::new(
                        Suit::new(suit), Value::new(value)
                    )
                );
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