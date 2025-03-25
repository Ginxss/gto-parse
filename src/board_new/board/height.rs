use super::Board;
use crate::board_new::rank::RankHeight;

pub enum BoardHeight {
    TripleBW,
    DoubleBW,
    SingleBW,
    Middling,
    Low,
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
