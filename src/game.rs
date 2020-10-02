// The game logic
use crate::card::Card;
use crate::player::Player;

pub struct Poker {
    players: Vec<Player>,
    deck: Vec<Card>,
    public_cards: Vec<Card>,
    button: usize,
    pot: u32,
    big_blind: u32,
    ante: u32,
}
/* impl Poker {
    pub fn new(number_of_player: u32, buy_in: u32, big_blind: u32, ante: u32) -> Poker {
        let mut players = vec![];
        for _ in 0..(number_of_player - 1) {
            players.push(Player::new(buy_in));
        }

        Poker {
            players: players,

        }
    }
    pub fn new_empty_table() -> Poker {
        Poker
    }
} */
