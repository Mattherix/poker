use poker::card::Card;
use poker::suit::Suit;
use poker::value::Value;

#[test]
fn create_valid_card() {
    Card {
        suit: Suit::Hearts,
        value: Value::new(1),
    };
}
#[test]
fn card_is_suit() {
    let king_of_hearts = Card {
        suit: Suit::Hearts,
        value: Value::new(13),
    };
    let queen_of_diamonds = Card {
        suit: Suit::Diamonds,
        value: Value::new(12),
    };
    let king_of_spades = Card {
        suit: Suit::Spades,
        value: Value::new(13),
    };
    let queen_of_clubs = Card {
        suit: Suit::Clubs,
        value: Value::new(12),
    };
    assert!(king_of_hearts.is_suit(Suit::Hearts));
    assert!(!king_of_hearts.is_suit(Suit::Diamonds));
    assert!(!king_of_hearts.is_suit(Suit::Spades));
    assert!(!king_of_hearts.is_suit(Suit::Clubs));

    assert!(!queen_of_diamonds.is_suit(Suit::Hearts));
    assert!(queen_of_diamonds.is_suit(Suit::Diamonds));
    assert!(!queen_of_diamonds.is_suit(Suit::Spades));
    assert!(!queen_of_diamonds.is_suit(Suit::Clubs));

    assert!(!king_of_spades.is_suit(Suit::Hearts));
    assert!(!king_of_spades.is_suit(Suit::Diamonds));
    assert!(king_of_spades.is_suit(Suit::Spades));
    assert!(!king_of_spades.is_suit(Suit::Clubs));

    assert!(!queen_of_clubs.is_suit(Suit::Hearts));
    assert!(!queen_of_clubs.is_suit(Suit::Diamonds));
    assert!(!queen_of_clubs.is_suit(Suit::Spades));
    assert!(queen_of_clubs.is_suit(Suit::Clubs));
}

#[test]
fn get_a_valid_deck() {
    let deck = Card::get_deck();
    for suit in ["Hearts", "Diamonds", "Spades", "Clubs"].iter() {
        for value in 1..14 {
            let card = Card {
                suit: Suit::new(suit),
                value: Value::new(value),
            };
            if !deck.contains(&card) {
                panic!("{} {} is missing", value, suit);
            }
        }
    }
}
