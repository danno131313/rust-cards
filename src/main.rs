extern crate cards;
use cards::deck::Deck;

fn main() {
    let deck = Deck::new();
    println!("{}", deck.cards[0]);
}
