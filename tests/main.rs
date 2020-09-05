#[cfg(test)]
mod tests {
    use poker::card::{Card, Suit, Value};
    #[test]
    fn create_valid_card() {
        Card {
            suit: Suit::Hearts,
            value: Value::new(1)
        };
    }
    #[test]
    fn card_is_suit() {
        let king_of_hearts = Card {
            suit: Suit::Hearts,
            value: Value::new(13)
        };
        let queen_of_diamonds = Card {
            suit: Suit::Diamonds,
            value: Value::new(12)
        };
        let king_of_spades = Card {
            suit: Suit::Spades,
            value: Value::new(13)
        };
        let queen_of_clubs = Card {
            suit: Suit::Clubs,
            value: Value::new(12)
        };
        assert!(king_of_hearts.is_suit(Suit::Hearts));
        assert!(queen_of_diamonds.is_suit(Suit::Diamonds));
        assert!(king_of_spades.is_suit(Suit::Spades));
        assert!(queen_of_clubs.is_suit(Suit::Clubs));

    }
    #[test]
    #[should_panic]
    fn wrong_value_card(){
        Card {
            suit: Suit::Hearts,
            value: Value::new(0)
        };
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
