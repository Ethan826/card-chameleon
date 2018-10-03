use card_chameleon::deck::Deck;

// https://www.codewars.com/kata/card-chameleon-a-cipher-with-playing-cards
// https://aarontoponce.org/wiki/card-ciphers/card-chameleon

fn main() {
    let mut rng = rand::thread_rng();

    let mut deck = Deck::new();
    deck.shuffle(&mut rng);
    println!("{:#?}", deck);
}
