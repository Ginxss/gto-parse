pub mod connection;
pub mod height;
pub mod pair;
pub mod suit;

use std::collections::BTreeSet;

use super::{card::Card, rank::RankHeight, BoardParseError};

#[derive(Debug)]
struct Board {
    pub cards: BTreeSet<Card>,
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
        let board = Board::try_from("3cAh7d").unwrap();

        assert_eq!(
            board.cards,
            BTreeSet::from([
                Card::try_from("7d").unwrap(),
                Card::try_from("3c").unwrap(),
                Card::try_from("Ah").unwrap(),
            ])
        );
    }

    #[test]
    fn test_num_rank_height() {
        let board = Board::try_from("7h4c7s").unwrap();
        assert_eq!(board.num_rank_height(&RankHeight::Broadway), 0);
        assert_eq!(board.num_rank_height(&RankHeight::Middling), 2);
        assert_eq!(board.num_rank_height(&RankHeight::Low), 1);

        let board = Board::try_from("AhKhTs").unwrap();
        assert_eq!(board.num_rank_height(&RankHeight::Broadway), 3);
        assert_eq!(board.num_rank_height(&RankHeight::Middling), 0);
        assert_eq!(board.num_rank_height(&RankHeight::Low), 0);

        let board = Board::try_from("3d8cJh").unwrap();
        assert_eq!(board.num_rank_height(&RankHeight::Broadway), 1);
        assert_eq!(board.num_rank_height(&RankHeight::Middling), 1);
        assert_eq!(board.num_rank_height(&RankHeight::Low), 1);

        let board = Board::try_from("JhTh2h").unwrap();
        assert_eq!(board.num_rank_height(&RankHeight::Broadway), 2);
        assert_eq!(board.num_rank_height(&RankHeight::Middling), 0);
        assert_eq!(board.num_rank_height(&RankHeight::Low), 1);
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_1() {
        Board::try_from("ÄsKsTd").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_2() {
        Board::try_from("AsKsÜd").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_3() {
        Board::try_from(" AsKsTd").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_4() {
        Board::try_from("AsKsTd ").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_5() {
        Board::try_from("invalid").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_6() {
        Board::try_from("128a").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_7() {
        Board::try_from("129ab").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_8() {
        Board::try_from("130abc").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_9() {
        Board::try_from("abc131").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_10() {
        Board::try_from("14c2h2h").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_11() {
        Board::try_from("15h5h5h").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_12() {
        Board::try_from("11c6h5s").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_13() {
        Board::try_from("13c6h5s").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_14() {
        Board::try_from("229h5s").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_15() {
        Board::try_from("19h6h6h").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_16() {
        Board::try_from("As Kh Td").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_17() {
        Board::try_from("10c6h5s").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_18() {
        Board::try_from("KsAc-4c6h").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_19() {
        Board::try_from("KsAc").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_20() {
        Board::try_from("KsAc66").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_21() {
        Board::try_from("SsAc-4c").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_22() {
        Board::try_from("KsAc-4c6h6d").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_23() {
        Board::try_from("KsAc-4h9").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_24() {
        Board::try_from("KsAc-4").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_25() {
        Board::try_from("ksAc-4h").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_26() {
        Board::try_from("Kscc-4h").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_board_from_invalid_board_27() {
        Board::try_from("Ks").unwrap();
    }
}
