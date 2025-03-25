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
    fn test_rank_from() {
        todo!();
    }
}
