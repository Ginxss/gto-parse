use crate::board_new::{card::Card, rank::RankHeight};

use super::Board;

pub enum BoardConnection {
    Disconnected,
    Gutshot,
    OESD,
    Wheel,
    Straight,
}

impl Board {
    pub fn is_normal_straight_possible(&self) -> bool {
        self.get_highest_card() - self.get_lowest_card() <= 4
    }

    pub fn is_wheel_possible(&self) -> bool {
        self.num_rank_height(&RankHeight::Wheel) == 3
    }

    pub fn is_any_straight_possible(&self) -> bool {
        self.is_normal_straight_possible() && self.is_wheel_possible()
    }

    pub fn is_only_oesd_possible(&self) -> bool {
        if self.is_any_straight_possible() {
            return false;
        }

        // TODO: Include double gutter AJ8 in Test!
        Card::get_distances(self.cards.iter().filter(|card| !card.is_ace()))
            .into_iter()
            .any(|diff| diff <= 3)
    }

    pub fn is_only_gutshot_possible(&self) -> bool {
        let distances = Card::get_distances(self.cards.iter());
        let min_distance = *distances.iter().min().unwrap();

        if min_distance == 4 {
            true
        } else if min_distance < 4 {
            false
        } else {
            self.get_highest_card().is_ace()
        }
    }

    pub fn is_disconnected(&self) -> bool {
        if self.get_highest_card().is_ace() {
            return false;
        }

        let distances = Card::get_distances(self.cards.iter());
        let min_distance = distances.iter().min().unwrap();
        if *min_distance <= 4 {
            return false;
        }

        true
    }

    fn get_lowest_card(&self) -> &Card {
        self.cards.first().unwrap()
    }

    fn get_highest_card(&self) -> &Card {
        self.cards.last().unwrap()
    }
}
