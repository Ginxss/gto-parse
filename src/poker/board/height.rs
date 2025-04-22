use super::Board;
use crate::poker::{rank::RankHeight, ParseError};

pub enum BoardHeight {
    TripleBW,
    DoubleBW,
    SingleBW,
    Middling,
    Low,
}

impl TryFrom<&str> for BoardHeight {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<BoardHeight, ParseError> {
        match s {
            "1BW" => Ok(BoardHeight::SingleBW),
            "2BW" => Ok(BoardHeight::DoubleBW),
            "3BW" => Ok(BoardHeight::TripleBW),
            "MID" => Ok(BoardHeight::Middling),
            "LOW" => Ok(BoardHeight::Low),
            _ => Err(ParseError::str("height", s)),
        }
    }
}

impl Board {
    pub fn is_3bw(&self) -> bool {
        self.num_rank_height(&RankHeight::Broadway) == 3
    }

    pub fn is_2bw(&self) -> bool {
        self.num_rank_height(&RankHeight::Broadway) == 2
    }

    pub fn is_1bw(&self) -> bool {
        self.num_rank_height(&RankHeight::Broadway) == 1
    }

    pub fn is_middling(&self) -> bool {
        self.num_rank_height(&RankHeight::Broadway) == 0
            && self.num_rank_height(&RankHeight::Middling) > 0
    }

    pub fn is_low(&self) -> bool {
        self.num_rank_height(&RankHeight::Low) == 3
    }

    pub fn is_height(&self, height: &BoardHeight) -> bool {
        match height {
            BoardHeight::TripleBW => self.is_3bw(),
            BoardHeight::DoubleBW => self.is_2bw(),
            BoardHeight::SingleBW => self.is_1bw(),
            BoardHeight::Middling => self.is_middling(),
            BoardHeight::Low => self.is_low(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_1bw() {
        assert!(Board::try_from("Ac9h6s").unwrap().is_1bw());
        assert!(Board::try_from("Ac5h4s").unwrap().is_1bw());
        assert!(Board::try_from("Jc6h6s").unwrap().is_1bw());
        assert!(Board::try_from("Tc9h2s").unwrap().is_1bw());
        assert!(!Board::try_from("9c8h6s").unwrap().is_1bw());
        assert!(!Board::try_from("Kh4hQh").unwrap().is_1bw());
        assert!(!Board::try_from("KcJhQs").unwrap().is_1bw());
        assert!(!Board::try_from("3c2h6s").unwrap().is_1bw());
        assert!(!Board::try_from("3c3h3s").unwrap().is_1bw());
    }

    #[test]
    fn test_is_2bw() {
        assert!(Board::try_from("Kc4hQs").unwrap().is_2bw());
        assert!(Board::try_from("AcTh6s").unwrap().is_2bw());
        assert!(Board::try_from("ThJh2d").unwrap().is_2bw());
        assert!(Board::try_from("ThTd5c").unwrap().is_2bw());
        assert!(!Board::try_from("Tc9h6s").unwrap().is_2bw());
        assert!(!Board::try_from("9h2h6h").unwrap().is_2bw());
        assert!(!Board::try_from("7h7c7d").unwrap().is_2bw());
        assert!(!Board::try_from("3c2h6s").unwrap().is_2bw());
        assert!(!Board::try_from("AhKdQd").unwrap().is_2bw());
    }

    #[test]
    fn test_is_3bw() {
        assert!(Board::try_from("KcJhQs").unwrap().is_3bw());
        assert!(Board::try_from("AcKhTs").unwrap().is_3bw());
        assert!(Board::try_from("JdKdTd").unwrap().is_3bw());
        assert!(Board::try_from("KhKdTh").unwrap().is_3bw());
        assert!(Board::try_from("QhQdQc").unwrap().is_3bw());
        assert!(!Board::try_from("AcKh6s").unwrap().is_3bw());
        assert!(!Board::try_from("Tc8s2s").unwrap().is_3bw());
        assert!(!Board::try_from("Jh8h8d").unwrap().is_3bw());
        assert!(!Board::try_from("3c2h5s").unwrap().is_3bw());
        assert!(!Board::try_from("7h8h9h").unwrap().is_3bw());
        assert!(!Board::try_from("2h2c2s").unwrap().is_3bw());
    }

    #[test]
    fn test_is_middling() {
        assert!(Board::try_from("9h3c2c").unwrap().is_middling());
        assert!(Board::try_from("8h7c2s").unwrap().is_middling());
        assert!(Board::try_from("9h8h7h").unwrap().is_middling());
        assert!(Board::try_from("8h8d2h").unwrap().is_middling());
        assert!(Board::try_from("7h7d7s").unwrap().is_middling());
        assert!(!Board::try_from("9h3cAc").unwrap().is_middling());
        assert!(!Board::try_from("6h3c2c").unwrap().is_middling());
        assert!(!Board::try_from("2h3c3h").unwrap().is_middling());
        assert!(!Board::try_from("TdThTs").unwrap().is_middling());
        assert!(!Board::try_from("AcKh6s").unwrap().is_middling());
        assert!(!Board::try_from("AcKhTs").unwrap().is_middling());
    }

    #[test]
    fn test_is_low() {
        assert!(Board::try_from("6h5h4h").unwrap().is_low());
        assert!(Board::try_from("5c2h3s").unwrap().is_low());
        assert!(Board::try_from("5c2h2s").unwrap().is_low());
        assert!(Board::try_from("2h6d6h").unwrap().is_low());
        assert!(Board::try_from("3h3c3s").unwrap().is_low());
        assert!(!Board::try_from("Ac2h3s").unwrap().is_low());
        assert!(!Board::try_from("Jh8s4s").unwrap().is_low());
        assert!(!Board::try_from("Tc8sKs").unwrap().is_low());
        assert!(!Board::try_from("7c8s6s").unwrap().is_low());
        assert!(!Board::try_from("6h6d7c").unwrap().is_low());
        assert!(!Board::try_from("JcJsJs").unwrap().is_low());
    }

    #[test]
    fn test_is_height() {
        assert!(Board::try_from("Qd8h2s")
            .unwrap()
            .is_height(&BoardHeight::SingleBW));
        assert!(Board::try_from("QdJh2s")
            .unwrap()
            .is_height(&BoardHeight::DoubleBW));
        assert!(Board::try_from("QdJhTs")
            .unwrap()
            .is_height(&BoardHeight::TripleBW));
        assert!(Board::try_from("9d8h2s")
            .unwrap()
            .is_height(&BoardHeight::Middling));
        assert!(Board::try_from("3d4h4s")
            .unwrap()
            .is_height(&BoardHeight::Low));
    }
}
