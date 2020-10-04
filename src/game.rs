// The game logic
use crate::card::Card;
use crate::player::Player;
use std::collections::HashSet;

pub struct Poker {
    pub players: HashSet<Player>,
    pub deck: HashSet<Card>,
    pub public_cards: HashSet<Card>,
    pub button: usize,
    pub pot: u32,
    pub big_blind: u32,
    pub ante: u32,
}
impl Poker {
    pub fn new(number_of_player: u32, buy_in: u32, big_blind: u32, ante: u32) -> Poker {
        let mut players = HashSet::new();
        for _ in 0..(number_of_player - 1) {
            players.insert(Player::new(buy_in));
        }

        Poker {
            players,
            deck: Card::get_deck(),
            public_cards: HashSet::new(),
            button: 0,
            pot: 0,
            big_blind,
            ante,
        }
    }
    pub fn new_empty_table() -> Poker {
        Poker {
            players: HashSet::new(),
            deck: Card::get_deck(),
            public_cards: HashSet::new(),
            button: 0,
            pot: 0,
            big_blind: 100,
            ante: 10,
        }
    }
}
