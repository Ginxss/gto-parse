use std::collections::HashSet;

use super::Board;

pub enum BoardSuit {
    Rainbow,
    Twotone,
    Montone,
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
