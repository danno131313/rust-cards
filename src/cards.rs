use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.value, self.suit)
    }
}

impl Card {
    /// Returns whether or not a card is a face card.
    pub fn is_face(&self) -> bool {
        self.value == Value::Ace || self.value == Value::Jack || self.value == Value::Queen || self.value == Value::King
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
