// The cards used by the poker game

#[derive(PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
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
    pub fn is_suit(&self, suit: Suit) -> bool {
        if self.suit == suit {
            true
        } else {
            false
        }
    }
}