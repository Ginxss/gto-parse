use super::BoardParseError;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Suit {
    Spade,
    Club,
    Diamond,
    Heart,
}

impl TryFrom<char> for Suit {
    type Error = BoardParseError;

    fn try_from(c: char) -> Result<Suit, BoardParseError> {
        match c {
            's' => Ok(Suit::Spade),
            'c' => Ok(Suit::Club),
            'd' => Ok(Suit::Diamond),
            'h' => Ok(Suit::Heart),
            _ => Err(BoardParseError::char("suit", c)),
        }
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
