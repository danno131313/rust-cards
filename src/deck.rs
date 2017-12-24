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
        let mut cards: Vec<Card> = Vec::with_capacity(52);
        let suits  = [Hearts, Spades, Diamonds, Clubs];
        let values = [Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King];
        for &suit in suits.into_iter() {
            for &value in values.into_iter() {
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
        let cards: Vec<Card> = Vec::with_capacity(52);
        Deck { cards }
    }
    
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn show(&self, idx: usize) -> &Card {
        self.cards.get(idx).expect("No card there!")
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_bottom(&mut self, card: Card) {
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

    pub fn draw_bottom(&mut self) -> Option<Card> {
        match self.cards.len() {
            0 => None,
            _x => Some(self.cards.remove(_x - 1)),
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards);
    }
}
