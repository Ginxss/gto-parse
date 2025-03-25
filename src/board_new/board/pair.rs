use std::collections::HashSet;

use super::Board;

pub enum BoardPair {
    NotPaired,
    Paired,
    Trips,
}

impl Board {
    pub fn is_not_paired(&self) -> bool {
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
            BoardPair::NotPaired => self.is_not_paired(),
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
