extern crate rand;
pub mod cards;

pub mod deck {
    use cards::Card;
    use cards::Suit::*;
    use cards::Value::*;
    use rand::{Rng,thread_rng};

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
            Deck { cards }
        }

        pub fn new_empty() -> Deck {
            let mut cards: Vec<Card> = Vec::new();
            Deck { cards }
        }

        pub fn add(&mut self, card: Card) {
            self.cards.push(card);
        }

        pub fn add_deck(&mut self, deck: Deck) {
            for card in deck.cards {
                self.cards.push(card);
            }
        }

        pub fn draw(&mut self) -> Card {
            self.cards.remove(0)
        }

        pub fn shuffle(&mut self) {
            let mut rng = thread_rng();
            for _ in 0..10 {
                for i in 0..self.cards.len() {
                    self.cards.swap(i, rng.gen_range(0,52));
                }
            }
        }
    }
}
