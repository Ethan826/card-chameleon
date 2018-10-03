use regex::Regex;
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn get_letter(&self) -> char {
        use self::Rank::*;
        use self::Suit::*;

        match self.suit {
            Hearts | Spades => match self.rank {
                Ace => 'A',
                Two => 'B',
                Three => 'C',
                Four => 'D',
                Five => 'E',
                Six => 'F',
                Seven => 'G',
                Eight => 'H',
                Nine => 'I',
                Ten => 'J',
                Jack => 'K',
                Queen => 'L',
                King => 'M',
            },
            Clubs | Diamonds => match self.rank {
                Ace => 'N',
                Two => 'O',
                Three => 'P',
                Four => 'Q',
                Five => 'R',
                Six => 'S',
                Seven => 'T',
                Eight => 'U',
                Nine => 'V',
                Ten => 'W',
                Jack => 'X',
                Queen => 'Y',
                King => 'Z',
            },
        }
    }

    pub fn is_black(&self) -> bool {
        self.suit == Suit::Spades || self.suit == Suit::Clubs
    }

    pub fn is_red(&self) -> bool {
        self.suit == Suit::Hearts || self.suit == Suit::Diamonds
    }
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        use self::Rank::*;
        use self::Suit::*;

        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<rank>[0-9]{1,2}|[AJKQ])(?P<suit>[CDHS])?").unwrap();
        }

        let caps = RE.captures(input).ok_or("Invalid card specified")?;

        let rank = match &caps
            .get("rank")
            .ok_or("Invalid card rank specified")?
            .as_str()
        {
            "A" => Ace,
            "2" => Two,
            "3" => Three,
            "4" => Four,
            "5" => Five,
            "6" => Six,
            "7" => Seven,
            "8" => Eight,
            "9" => Nine,
            "10" => Ten,
            "J" => Jack,
            "Q" => Queen,
            "K" => King,
        };

        let suit = match caps
            .get("suit")
            .ok_or("Invalid card suit specified")?
            .as_str()
        {
            "C" => Clubs,
            "D" => Diamonds,
            "H" => Hearts,
            "S" => Spades,
        };

        Ok(Card { rank, suit })
    }
}

// =================================================================================================
// Tests
// =================================================================================================

#[test]
fn test_is_black() {
    let mut card = Card {
        rank: Rank::Two,
        suit: Suit::Clubs,
    };

    assert!(card.is_black());

    card = Card {
        rank: Rank::Two,
        suit: Suit::Hearts,
    };

    assert!(!card.is_black());

    card = Card {
        rank: Rank::Two,
        suit: Suit::Spades,
    };

    assert!(card.is_black());

    card = Card {
        rank: Rank::Two,
        suit: Suit::Diamonds,
    };

    assert!(!card.is_black());
}

#[test]
fn test_is_red() {
    let mut card = Card {
        rank: Rank::Two,
        suit: Suit::Clubs,
    };

    assert!(!card.is_red());

    card = Card {
        rank: Rank::Two,
        suit: Suit::Hearts,
    };

    assert!(card.is_red());

    card = Card {
        rank: Rank::Two,
        suit: Suit::Spades,
    };

    assert!(!card.is_red());

    card = Card {
        rank: Rank::Two,
        suit: Suit::Diamonds,
    };

    assert!(card.is_red());
}

#[test]
fn get_letter() {
    assert_eq!(
        Card {
            rank: Rank::Seven,
            suit: Suit::Clubs
        }
        .get_letter(),
        'T'
    );
}
