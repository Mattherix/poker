// The game logic

struct Poker {
    player: vec![Player],
    deck: vec![Card],
    public_cards: vec![Card],
    button: usize,
    pot: u32,
    big_blind: u32,
    ante: u32
}
