use std::collections::HashSet;

use crate::poker::ParseError;

use super::Board;

pub enum BoardSuit {
    Rainbow,
    Twotone,
    Montone,
}

impl TryFrom<&str> for BoardSuit {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<BoardSuit, ParseError> {
        match s {
            "R" => Ok(BoardSuit::Rainbow),
            "T" => Ok(BoardSuit::Twotone),
            "M" => Ok(BoardSuit::Montone),
            _ => Err(ParseError::str("suit", s)),
        }
    }
}

impl Board {
    pub fn is_rainbow(&self) -> bool {
        self.num_unique_suits() == 3
    }

    pub fn is_twotone(&self) -> bool {
        self.num_unique_suits() == 2
    }

    pub fn is_monotone(&self) -> bool {
        self.num_unique_suits() == 1
    }

    pub fn is_suit(&self, suit: &BoardSuit) -> bool {
        match suit {
            BoardSuit::Rainbow => self.is_rainbow(),
            BoardSuit::Twotone => self.is_twotone(),
            BoardSuit::Montone => self.is_monotone(),
        }
    }

    fn num_unique_suits(&self) -> usize {
        self.cards
            .iter()
            .map(|card| &card.suit)
            .collect::<HashSet<_>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rainbow() {
        assert!(Board::try_from("Qc9s7h").unwrap().is_rainbow());
        assert!(Board::try_from("9c9d7h").unwrap().is_rainbow());
        assert!(Board::try_from("2c2h2d").unwrap().is_rainbow());
        assert!(!Board::try_from("Qc9s7s").unwrap().is_rainbow());
        assert!(!Board::try_from("Jc9d7d").unwrap().is_rainbow());
    }

    #[test]
    fn test_is_twotone() {
        assert!(Board::try_from("Qc9h7h").unwrap().is_twotone());
        assert!(Board::try_from("As2c5s").unwrap().is_twotone());
        assert!(Board::try_from("5d7d7c").unwrap().is_twotone());
        assert!(!Board::try_from("As2c5d").unwrap().is_twotone());
        assert!(!Board::try_from("5d7d8d").unwrap().is_twotone());
    }

    #[test]
    fn test_is_monotone() {
        assert!(Board::try_from("Qc9c7c").unwrap().is_monotone());
        assert!(Board::try_from("2s3sJs").unwrap().is_monotone());
        assert!(Board::try_from("2hAhKh").unwrap().is_monotone());
        assert!(!Board::try_from("2s3sJd").unwrap().is_monotone());
        assert!(!Board::try_from("2hAdKc").unwrap().is_monotone());
    }

    #[test]
    fn test_is_suit() {
        assert!(Board::try_from("Qs9s6s")
            .unwrap()
            .is_suit(&BoardSuit::Montone));
        assert!(Board::try_from("Qc9c7d")
            .unwrap()
            .is_suit(&BoardSuit::Twotone));
        assert!(Board::try_from("Kc5s2h")
            .unwrap()
            .is_suit(&BoardSuit::Rainbow));
    }

    #[test]
    fn test_num_unique_suits() {
        assert_eq!(Board::try_from("JcTc7c").unwrap().num_unique_suits(), 1);
        assert_eq!(Board::try_from("JcTh7h").unwrap().num_unique_suits(), 2);
        assert_eq!(Board::try_from("JcTs7h").unwrap().num_unique_suits(), 3);
    }
}
