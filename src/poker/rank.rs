use std::ops::Sub;

use super::BoardParseError;

pub enum RankHeight {
    Broadway,
    Middling,
    Low,
    Wheel,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Rank {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Rank {
    type Error = BoardParseError;

    fn try_from(c: char) -> Result<Rank, BoardParseError> {
        match c {
            '2' => Ok(Rank::_2),
            '3' => Ok(Rank::_3),
            '4' => Ok(Rank::_4),
            '5' => Ok(Rank::_5),
            '6' => Ok(Rank::_6),
            '7' => Ok(Rank::_7),
            '8' => Ok(Rank::_8),
            '9' => Ok(Rank::_9),
            'T' => Ok(Rank::T),
            'J' => Ok(Rank::J),
            'Q' => Ok(Rank::Q),
            'K' => Ok(Rank::K),
            'A' => Ok(Rank::A),
            _ => Err(BoardParseError::char("rank", c)),
        }
    }
}

impl Rank {
    pub fn is_bw(&self) -> bool {
        match self {
            Rank::A | Rank::K | Rank::Q | Rank::J | Rank::T => true,
            _ => false,
        }
    }

    pub fn is_middling(&self) -> bool {
        match self {
            Rank::_9 | Rank::_8 | Rank::_7 => true,
            _ => false,
        }
    }

    pub fn is_low(&self) -> bool {
        match self {
            Rank::_6 | Rank::_5 | Rank::_4 | Rank::_3 | Rank::_2 => true,
            _ => false,
        }
    }

    pub fn is_wheel(&self) -> bool {
        match self {
            Rank::A | Rank::_2 | Rank::_3 | Rank::_4 | Rank::_5 => true,
            _ => false,
        }
    }

    pub fn is_height(&self, height: &RankHeight) -> bool {
        match height {
            RankHeight::Broadway => self.is_bw(),
            RankHeight::Middling => self.is_middling(),
            RankHeight::Low => self.is_low(),
            RankHeight::Wheel => self.is_wheel(),
        }
    }

    pub fn get_heights(&self) -> Vec<RankHeight> {
        todo!()
    }
}

impl Sub for Rank {
    type Output = i32;

    fn sub(self, rhs: Rank) -> i32 {
        self as i32 - rhs as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_try_from() {
        assert_eq!(Rank::try_from('2').unwrap(), Rank::_2);
        assert_eq!(Rank::try_from('3').unwrap(), Rank::_3);
        assert_eq!(Rank::try_from('4').unwrap(), Rank::_4);
        assert_eq!(Rank::try_from('5').unwrap(), Rank::_5);
        assert_eq!(Rank::try_from('6').unwrap(), Rank::_6);
        assert_eq!(Rank::try_from('7').unwrap(), Rank::_7);
        assert_eq!(Rank::try_from('8').unwrap(), Rank::_8);
        assert_eq!(Rank::try_from('9').unwrap(), Rank::_9);
        assert_eq!(Rank::try_from('T').unwrap(), Rank::T);
        assert_eq!(Rank::try_from('J').unwrap(), Rank::J);
        assert_eq!(Rank::try_from('Q').unwrap(), Rank::Q);
        assert_eq!(Rank::try_from('K').unwrap(), Rank::K);
        assert_eq!(Rank::try_from('A').unwrap(), Rank::A);
    }

    #[test]
    fn test_rank_is_bw() {
        assert!(Rank::A.is_bw());
        assert!(Rank::K.is_bw());
        assert!(Rank::Q.is_bw());
        assert!(Rank::J.is_bw());
        assert!(Rank::T.is_bw());
        assert!(!Rank::_9.is_bw());
        assert!(!Rank::_8.is_bw());
        assert!(!Rank::_7.is_bw());
        assert!(!Rank::_6.is_bw());
        assert!(!Rank::_5.is_bw());
        assert!(!Rank::_4.is_bw());
        assert!(!Rank::_3.is_bw());
        assert!(!Rank::_2.is_bw());
    }

    #[test]
    fn test_rank_is_middling() {
        assert!(!Rank::A.is_middling());
        assert!(!Rank::K.is_middling());
        assert!(!Rank::Q.is_middling());
        assert!(!Rank::J.is_middling());
        assert!(!Rank::T.is_middling());
        assert!(Rank::_9.is_middling());
        assert!(Rank::_8.is_middling());
        assert!(Rank::_7.is_middling());
        assert!(!Rank::_6.is_middling());
        assert!(!Rank::_5.is_middling());
        assert!(!Rank::_4.is_middling());
        assert!(!Rank::_3.is_middling());
        assert!(!Rank::_2.is_middling());
    }

    #[test]
    fn test_rank_is_low() {
        assert!(!Rank::A.is_middling());
        assert!(!Rank::K.is_middling());
        assert!(!Rank::Q.is_middling());
        assert!(!Rank::J.is_middling());
        assert!(!Rank::T.is_middling());
        assert!(!Rank::_9.is_middling());
        assert!(!Rank::_8.is_middling());
        assert!(!Rank::_7.is_middling());
        assert!(Rank::_6.is_middling());
        assert!(Rank::_5.is_middling());
        assert!(Rank::_4.is_middling());
        assert!(Rank::_3.is_middling());
        assert!(Rank::_2.is_middling());
    }

    #[test]
    fn test_is_wheel() {
        assert!(Rank::A.is_middling());
        assert!(!Rank::K.is_middling());
        assert!(!Rank::Q.is_middling());
        assert!(!Rank::J.is_middling());
        assert!(!Rank::T.is_middling());
        assert!(!Rank::_9.is_middling());
        assert!(!Rank::_8.is_middling());
        assert!(!Rank::_7.is_middling());
        assert!(!Rank::_6.is_middling());
        assert!(Rank::_5.is_middling());
        assert!(Rank::_4.is_middling());
        assert!(Rank::_3.is_middling());
        assert!(Rank::_2.is_middling());
    }

    #[test]
    fn test_is_height() {
        assert!(Rank::A.is_height(&RankHeight::Broadway));
        assert!(!Rank::A.is_height(&RankHeight::Middling));
        assert!(!Rank::A.is_height(&RankHeight::Low));
        assert!(Rank::A.is_height(&RankHeight::Wheel));

        assert!(Rank::K.is_height(&RankHeight::Broadway));
        assert!(!Rank::K.is_height(&RankHeight::Middling));
        assert!(!Rank::K.is_height(&RankHeight::Low));
        assert!(!Rank::K.is_height(&RankHeight::Wheel));

        assert!(!Rank::_8.is_height(&RankHeight::Broadway));
        assert!(Rank::_8.is_height(&RankHeight::Middling));
        assert!(!Rank::_8.is_height(&RankHeight::Low));
        assert!(!Rank::_8.is_height(&RankHeight::Wheel));

        assert!(!Rank::_6.is_height(&RankHeight::Broadway));
        assert!(!Rank::_6.is_height(&RankHeight::Middling));
        assert!(Rank::_6.is_height(&RankHeight::Low));
        assert!(!Rank::_6.is_height(&RankHeight::Wheel));

        assert!(!Rank::_5.is_height(&RankHeight::Broadway));
        assert!(!Rank::_5.is_height(&RankHeight::Middling));
        assert!(Rank::_5.is_height(&RankHeight::Low));
        assert!(Rank::_5.is_height(&RankHeight::Wheel));
    }

    #[test]
    fn test_rank_sub() {
        assert_eq!(Rank::A - Rank::K, 1);
        assert_eq!(Rank::A - Rank::T, 5);
        assert_eq!(Rank::J - Rank::T, 1);
        assert_eq!(Rank::J - Rank::_3, 7);
        assert_eq!(Rank::_2 - Rank::_4, -2);
        assert_eq!(Rank::_8 - Rank::_4, 4);
    }

    #[test]
    #[should_panic]
    fn test_invalid_rank_try_from_1() {
        Rank::try_from('a').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_rank_try_from_2() {
        Rank::try_from('1').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_rank_try_from_3() {
        Rank::try_from('0').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_rank_try_from_4() {
        Rank::try_from('ä').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_rank_try_from_5() {
        Rank::try_from('r').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_rank_try_from_6() {
        Rank::try_from('^').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_rank_try_from_7() {
        Rank::try_from('\\').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_rank_try_from_8() {
        Rank::try_from('´').unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_rank_try_from_9() {
        Rank::try_from(' ').unwrap();
    }
}
