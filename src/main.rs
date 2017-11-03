extern crate cards;
use cards::deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let card = deck.draw();
    let card2 = deck.draw();
    println!("{}, {}", card, card2);
}
