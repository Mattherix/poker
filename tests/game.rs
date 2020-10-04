use poker::game::Poker;
use poker::player::Player;

#[test]
fn create_valid_poker() {
    Poker::new(9, 10_000, 100, 0);
}
#[test]
fn create_valid_empty_poker() {
    let mut poker_game = Poker::new_empty_table();
    poker_game.players.insert(Player::new(10_000));
}