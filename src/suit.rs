// The suit of each card
#[derive(Eq, PartialEq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}
impl Suit {
    pub fn new(suit: &str) -> Suit {
        match suit {
            "Hearts" => Suit::Hearts,
            "Diamonds" => Suit::Diamonds,
            "Spades" => Suit::Spades,
            "Clubs" => Suit::Clubs,
            _ => panic!("Card can only be Hearts, Diamonds, Spades or Clubs"),
        }
    }
}
