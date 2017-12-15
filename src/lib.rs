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
            let cards: Vec<Card> = Vec::new();
            Deck { cards }
        }
        
        pub fn len(&self) -> usize {
            (self.cards.len())
        }

        pub fn show(&self, idx: usize) -> &Card {
            self.cards.get(idx).expect("No card there!")
        }

        pub fn add(&mut self, card: Card) {
            self.cards.push(card);
        }

        pub fn add_back(&mut self, card: Card) {
            self.cards.insert(0, card);
        }

        pub fn add_deck(&mut self, deck: &mut Deck) {
            for _ in 0..deck.len() {
                self.cards.push(deck.draw().expect("Deck is empty!"));
            }
        }

        pub fn draw(&mut self) -> Option<Card> {
            match self.cards.len() {
                0 => None,
                _ => Some(self.cards.remove(0)),
            }
        }

        pub fn bottom_draw(&mut self) -> Option<Card> {
            match self.cards.len() {
                0 => None,
                _x => Some(self.cards.remove(_x - 1)),
            }
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
