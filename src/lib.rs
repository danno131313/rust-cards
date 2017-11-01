mod cards;

pub mod deck {
    use cards::Card;
    use cards::Suit::*;
    use cards::Value::*;

    #[derive(Debug)]
    pub struct Deck {
        pub cards: Vec<Card>,
    }

    impl Deck {
        pub fn new() -> Deck {
            let mut cards = Vec::new();
            let suits  = [Hearts, Spades, Diamonds, Clubs];
            let values = [Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King];
            for &suit in suits.iter() {
                for &value in values.iter() {
                    let card = Card {
                        suit:  suit,
                        value: value,
                    };
                    cards.push(card);
                }
            }
            return Deck {cards};
        }
    }
}
