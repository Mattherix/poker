// The player struct
use crate::card::Card;

#[derive(Eq, PartialEq, Hash)]
pub struct Player {
    stack: u32,
    private_cards: Vec<Card>,
    game_played: u32,
    flop_seen: u32,
}
impl Player {
    pub fn new(stack: u32) -> Player {
        Player {
            stack,
            private_cards: vec![],
            game_played: 0,
            flop_seen: 0,
        }
    }
}
