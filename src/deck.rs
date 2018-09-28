use crate::card::*;
use rand::{Rng, ThreadRng};

pub struct Deck([Card; 54]);

impl std::fmt::Debug for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0.iter().collect::<Vec<_>>())
    }
}

impl Deck {
    pub fn new() -> Self {
        use self::Rank::*;
        use self::Suit::*;

        Deck([
            Card {
                rank: Joker,
                suit: Black,
            },
            Card {
                rank: Joker,
                suit: Red,
            },
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: Two,
                suit: Hearts,
            },
            Card {
                rank: Three,
                suit: Hearts,
            },
            Card {
                rank: Four,
                suit: Hearts,
            },
            Card {
                rank: Five,
                suit: Hearts,
            },
            Card {
                rank: Six,
                suit: Hearts,
            },
            Card {
                rank: Seven,
                suit: Hearts,
            },
            Card {
                rank: Eight,
                suit: Hearts,
            },
            Card {
                rank: Nine,
                suit: Hearts,
            },
            Card {
                rank: Ten,
                suit: Hearts,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
            Card {
                rank: Queen,
                suit: Hearts,
            },
            Card {
                rank: King,
                suit: Hearts,
            },
            Card {
                rank: Ace,
                suit: Clubs,
            },
            Card {
                rank: Two,
                suit: Clubs,
            },
            Card {
                rank: Three,
                suit: Clubs,
            },
            Card {
                rank: Four,
                suit: Clubs,
            },
            Card {
                rank: Five,
                suit: Clubs,
            },
            Card {
                rank: Six,
                suit: Clubs,
            },
            Card {
                rank: Seven,
                suit: Clubs,
            },
            Card {
                rank: Eight,
                suit: Clubs,
            },
            Card {
                rank: Nine,
                suit: Clubs,
            },
            Card {
                rank: Ten,
                suit: Clubs,
            },
            Card {
                rank: Jack,
                suit: Clubs,
            },
            Card {
                rank: Queen,
                suit: Clubs,
            },
            Card {
                rank: King,
                suit: Clubs,
            },
            Card {
                rank: King,
                suit: Diamonds,
            },
            Card {
                rank: Queen,
                suit: Diamonds,
            },
            Card {
                rank: Jack,
                suit: Diamonds,
            },
            Card {
                rank: Ten,
                suit: Diamonds,
            },
            Card {
                rank: Nine,
                suit: Diamonds,
            },
            Card {
                rank: Eight,
                suit: Diamonds,
            },
            Card {
                rank: Seven,
                suit: Diamonds,
            },
            Card {
                rank: Six,
                suit: Diamonds,
            },
            Card {
                rank: Five,
                suit: Diamonds,
            },
            Card {
                rank: Four,
                suit: Diamonds,
            },
            Card {
                rank: Three,
                suit: Diamonds,
            },
            Card {
                rank: Two,
                suit: Diamonds,
            },
            Card {
                rank: Ace,
                suit: Diamonds,
            },
            Card {
                rank: King,
                suit: Spades,
            },
            Card {
                rank: Queen,
                suit: Spades,
            },
            Card {
                rank: Jack,
                suit: Spades,
            },
            Card {
                rank: Ten,
                suit: Spades,
            },
            Card {
                rank: Nine,
                suit: Spades,
            },
            Card {
                rank: Eight,
                suit: Spades,
            },
            Card {
                rank: Seven,
                suit: Spades,
            },
            Card {
                rank: Six,
                suit: Spades,
            },
            Card {
                rank: Five,
                suit: Spades,
            },
            Card {
                rank: Four,
                suit: Spades,
            },
            Card {
                rank: Three,
                suit: Spades,
            },
            Card {
                rank: Two,
                suit: Spades,
            },
            Card {
                rank: Ace,
                suit: Spades,
            },
        ])
    }

    pub fn shuffle(&mut self, rng: &mut ThreadRng) {
        rng.shuffle(&mut self.0);
    }
}
