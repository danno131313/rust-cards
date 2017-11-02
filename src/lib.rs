extern crate rand;
mod cards;

pub mod deck {
    use cards::Card;
    use cards::Suit::*;
    use cards::Value::*;
    use rand::Rng;

    #[derive(Debug)]
    pub struct Deck {
        cards: Vec<Card>,
    }

    impl Deck {
        pub fn new() -> Deck {
            let mut cards: Vec<Card> = Vec::new();
            let suits  = [Hearts, Spades, Diamonds, Clubs];
            let values = [Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King];
            for &suit in suits.iter() {
                for &value in values.iter() {
                    let card = Card {
                        suit,
                        value,
                    };
                    cards.push(card);
                }
            }
            Deck {cards}
        }

        pub fn draw(&mut self) -> Card {
            self.cards.remove(0)
        }

        pub fn shuffle(&mut self) {
            let mut 
        }
    }
}
