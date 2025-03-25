pub mod connection;
pub mod height;
pub mod pair;
pub mod suit;

use std::collections::BTreeSet;

use super::{card::Card, rank::RankHeight, BoardParseError};

#[derive(Debug)]
struct Board {
    cards: BTreeSet<Card>,
}

impl TryFrom<&str> for Board {
    type Error = BoardParseError;

    fn try_from(board_str: &str) -> Result<Board, BoardParseError> {
        let expected_length = 6;
        if board_str.len() != 6 {
            return Err(BoardParseError::str("flop", board_str));
        }

        let cards = (0..expected_length)
            .step_by(2)
            .map(|i| &board_str[i..i + 2])
            .map(|card_str| Card::try_from(card_str))
            .collect::<Result<_, _>>()?;

        Ok(Board { cards })
    }
}

impl Board {
    fn num_rank_height(&self, height: &RankHeight) -> usize {
        self.cards
            .iter()
            .filter(|card| card.is_height(height))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_from() {
        let board = Board::try_from("3cAh7d");
        println!("{:?}", board);
    }
}
