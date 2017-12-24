use cards::Card;
use cards::Suit::*;
use cards::Value::*;
use rand::{Rng,thread_rng};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a new, ordered deck of 52 cards.
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

    /// Creates a new, empty deck.
    pub fn new_empty() -> Deck {
        let cards: Vec<Card> = Vec::with_capacity(52);
        Deck { cards }
    }
    
    /// Returns the amount of cards in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns reference to a card given an index.
    pub fn show(&self, idx: usize) -> &Card {
        self.cards.get(idx).expect("No card there!")
    }

    /// Adds a card to the bottom of the deck.
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Adds a card to the front of the deck.
    pub fn add_bottom(&mut self, card: Card) {
        self.cards.insert(0, card);
    }

    /// Adds a deck of cards, one by one, to the bottom of this deck.
    pub fn add_deck(&mut self, deck: &mut Deck) {
        for _ in 0..deck.len() {
            self.cards.push(deck.draw().expect("Deck is empty!"));
        }
    }

    /// Draws one card from the front of the deck, returning an Option<Card>.
    pub fn draw(&mut self) -> Option<Card> {
        match self.cards.len() {
            0 => None,
            _ => Some(self.cards.remove(0)),
        }
    }

    /// Draws one card from the bottom of the deck, return an Option<Card>.
    pub fn draw_bottom(&mut self) -> Option<Card> {
        match self.cards.len() {
            0 => None,
            _x => Some(self.cards.remove(_x - 1)),
        }
    }

    /// Shuffles the deck.
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards);
    }
}
