extern crate cards;
use cards::deck::Deck;

fn main() {
    let mut deck1 = Deck::new();
    let mut deck2 = Deck::new();
    deck1.shuffle();
    deck2.shuffle();
    let card1 = deck1.draw();
    let card2 = deck2.draw();
    let same: bool = card1 == card2;
    println!("{}, {}\nSame: {}", card1, card2, same);
}
