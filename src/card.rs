#[derive(Debug, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    Red,
    Black,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
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
    Ace,
    Joker,
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn is_black(&self) -> bool {
        self.suit == Suit::Black || self.suit == Suit::Spades || self.suit == Suit::Clubs
    }

    pub fn is_red(&self) -> bool {
        self.suit == Suit::Red || self.suit == Suit::Hearts || self.suit == Suit::Diamonds
    }
}
