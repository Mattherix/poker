// The game logic
use crate::card::Card;
use crate::player::Player;

pub struct Poker {
    player: Vec<Player>,
    deck: Vec<Card>,
    public_cards: Vec<Card>,
    button: usize,
    pot: u32,
    big_blind: u32,
    ante: u32
}
