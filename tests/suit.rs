use poker::suit::Suit;

#[test]
fn create_valid_suit() {
    Suit::new("Hearts");
    Suit::new("Diamonds");
    Suit::new("Spades");
    Suit::new("Clubs");
}

#[test]
#[should_panic]
fn wrong_suit() {
    Suit::new("Pink");
}
