use std::fmt::{self, Display, Formatter};

use super::ParseError;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Suit {
    Spade,
    Club,
    Diamond,
    Heart,
}

const SPADE_CHAR: char = 's';
const CLUB_CHAR: char = 'c';
const DIAMOND_CHAR: char = 'd';
const HEART_CHAR: char = 'h';

impl TryFrom<char> for Suit {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Suit, ParseError> {
        match c {
            SPADE_CHAR => Ok(Suit::Spade),
            CLUB_CHAR => Ok(Suit::Club),
            DIAMOND_CHAR => Ok(Suit::Diamond),
            HEART_CHAR => Ok(Suit::Heart),
            _ => Err(ParseError::char("suit", c)),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Suit::Spade => SPADE_CHAR,
            Suit::Club => CLUB_CHAR,
            Suit::Diamond => DIAMOND_CHAR,
            Suit::Heart => HEART_CHAR,
        };

        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suit_try_from() {
        assert_eq!(Suit::try_from('h').unwrap(), Suit::Heart);
        assert_eq!(Suit::try_from('c').unwrap(), Suit::Club);
        assert_eq!(Suit::try_from('d').unwrap(), Suit::Diamond);
        assert_eq!(Suit::try_from('s').unwrap(), Suit::Spade);
    }

    #[test]
    #[should_panic]
    fn test_invalid_suit_try_from_2() {
        Suit::try_from('H').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_suit_try_from_3() {
        Suit::try_from('D').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_suit_try_from_4() {
        Suit::try_from('ö').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_suit_try_from_5() {
        Suit::try_from('ß').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_suit_try_from_6() {
        Suit::try_from(' ').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_suit_try_from_7() {
        Suit::try_from('\\').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_suit_try_from_8() {
        Suit::try_from('*').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_suit_try_from_9() {
        Suit::try_from('.').unwrap();
    }
}
