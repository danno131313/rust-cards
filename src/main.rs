extern crate cards;
use cards::deck::Deck;

fn main() {
    let mut deck1 = Deck::new();
    let mut deck2 = Deck::new();
    //deck1.shuffle();
    //deck2.shuffle();
    let mut pile = Deck::new_empty();
    pile.add(deck1.draw().expect(""));
    pile.add(deck1.draw().expect(""));
    deck1.add_deck(&mut pile);
    let card1 = deck1.draw().expect("Deck is empty!");
    let card2 = deck2.draw().expect("Deck is empty!");
    let same: bool = card1 == card2;
    println!("{}", same);
    for i in 0..deck1.len() {
        println!("{}", deck1.show(i));
    }
}
