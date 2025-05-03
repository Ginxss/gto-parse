use std::{collections::HashSet, str::FromStr};

use crate::poker::ParseError;

use super::Board;

pub enum BoardPair {
    Unpaired,
    Paired,
    Trips,
}

impl FromStr for BoardPair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(BoardPair::Unpaired),
            "P" => Ok(BoardPair::Paired),
            "T" => Ok(BoardPair::Trips),
            _ => Err(ParseError::str("pair", s)),
        }
    }
}

impl Board {
    pub fn is_unpaired(&self) -> bool {
        self.num_unique_ranks() == 3
    }

    pub fn is_paired(&self) -> bool {
        self.num_unique_ranks() == 2
    }

    pub fn is_trips(&self) -> bool {
        self.num_unique_ranks() == 1
    }

    pub fn is_pair(&self, pair: &BoardPair) -> bool {
        match pair {
            BoardPair::Unpaired => self.is_unpaired(),
            BoardPair::Paired => self.is_paired(),
            BoardPair::Trips => self.is_trips(),
        }
    }

    fn num_unique_ranks(&self) -> usize {
        self.cards
            .iter()
            .map(|card| &card.rank)
            .collect::<HashSet<_>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unpaired() {
        assert!(Board::try_from("Qc9s7h").unwrap().is_unpaired());
        assert!(Board::try_from("AcKdQd").unwrap().is_unpaired());
        assert!(Board::try_from("2c3c8c").unwrap().is_unpaired());
        assert!(!Board::try_from("2c2d8h").unwrap().is_unpaired());
        assert!(!Board::try_from("2c2d2h").unwrap().is_unpaired());
    }

    #[test]
    fn test_is_paired() {
        assert!(Board::try_from("Qc9s9h").unwrap().is_paired());
        assert!(Board::try_from("AcAdQh").unwrap().is_paired());
        assert!(Board::try_from("2h8c2c").unwrap().is_paired());
        assert!(!Board::try_from("AcAdAh").unwrap().is_paired());
        assert!(!Board::try_from("2h8cTc").unwrap().is_paired());
    }

    #[test]
    fn test_is_trips() {
        assert!(Board::try_from("QcQsQh").unwrap().is_trips());
        assert!(Board::try_from("2c2d2h").unwrap().is_trips());
        assert!(!Board::try_from("AcAd2h").unwrap().is_trips());
        assert!(!Board::try_from("2h8cTc").unwrap().is_trips());
    }

    #[test]
    fn test_is_pair() {
        assert!(Board::try_from("JcJsJh")
            .unwrap()
            .is_pair(&BoardPair::Trips));
        assert!(!Board::try_from("JcJsJh")
            .unwrap()
            .is_pair(&BoardPair::Paired));
        assert!(!Board::try_from("JcJsJh")
            .unwrap()
            .is_pair(&BoardPair::Unpaired));

        assert!(!Board::try_from("QcQd2h")
            .unwrap()
            .is_pair(&BoardPair::Trips));
        assert!(Board::try_from("QcQd2h")
            .unwrap()
            .is_pair(&BoardPair::Paired));
        assert!(!Board::try_from("QcQd2h")
            .unwrap()
            .is_pair(&BoardPair::Unpaired));

        assert!(!Board::try_from("2h8c9c")
            .unwrap()
            .is_pair(&BoardPair::Trips));
        assert!(!Board::try_from("2h8c9c")
            .unwrap()
            .is_pair(&BoardPair::Paired));
        assert!(Board::try_from("2h8c9c")
            .unwrap()
            .is_pair(&BoardPair::Unpaired));
    }

    #[test]
    fn test_num_unique_ranks() {
        assert_eq!(Board::try_from("2c2d2h").unwrap().num_unique_ranks(), 1);
        assert_eq!(Board::try_from("2c8c8d").unwrap().num_unique_ranks(), 2);
        assert_eq!(Board::try_from("2cKd8h").unwrap().num_unique_ranks(), 3);
    }
}
