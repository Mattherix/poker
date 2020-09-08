// The player struct
use crate::card::Card;

pub struct Player {
    stack: u32,
    private_cards: Vec<Card>,
    game_played: u32,
    flop_seen: u32
}