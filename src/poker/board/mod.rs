pub mod connection;
pub mod height;
pub mod pair;
pub mod suit;

use std::{
    collections::BTreeSet,
    fmt::{self, Display, Formatter},
};

use super::{card::Card, rank::RankHeight, ParseError};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Board {
    pub cards: BTreeSet<Card>,
}

impl TryFrom<&str> for Board {
    type Error = ParseError;

    fn try_from(board_str: &str) -> Result<Board, ParseError> {
        let expected_length = 6;
        if board_str.len() != 6 {
            return Err(ParseError::str("flop", board_str));
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

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s: String = self.cards.iter().map(|card| card.to_string()).collect();

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::poker::{rank::Rank, suit::Suit};

    use super::*;

    #[test]
    fn test_board_try_from() {
        let board = Board::try_from("3cAh7d").unwrap();

        assert_eq!(
            board.cards,
            BTreeSet::from([
                Card::try_from("7d").unwrap(),
                Card::try_from("3c").unwrap(),
                Card::try_from("Ah").unwrap(),
            ])
        );

        let card1 = board.cards.iter().nth(0).unwrap();
        let card2 = board.cards.iter().nth(1).unwrap();
        let card3 = board.cards.iter().nth(2).unwrap();

        assert_eq!(card1.rank, Rank::_3);
        assert_eq!(card1.suit, Suit::Club);

        assert_eq!(card2.rank, Rank::_7);
        assert_eq!(card2.suit, Suit::Diamond);

        assert_eq!(card3.rank, Rank::A);
        assert_eq!(card3.suit, Suit::Heart);
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
    fn test_invalid_board_try_from_1() {
        Board::try_from("ÄsKsTd").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_2() {
        Board::try_from("AsKsÜd").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_3() {
        Board::try_from(" AsKsTd").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_4() {
        Board::try_from("AsKsTd ").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_5() {
        Board::try_from("invalid").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_6() {
        Board::try_from("128a").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_7() {
        Board::try_from("129ab").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_8() {
        Board::try_from("130abc").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_9() {
        Board::try_from("abc131").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_10() {
        Board::try_from("14c2h2h").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_12() {
        Board::try_from("11c6h5s").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_14() {
        Board::try_from("229h5s").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_16() {
        Board::try_from("As Kh Td").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_17() {
        Board::try_from("10c6h5s").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_18() {
        Board::try_from("KsAc-4c6h").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_19() {
        Board::try_from("KsAc").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_20() {
        Board::try_from("KsAc66").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_21() {
        Board::try_from("SsAc-4c").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_22() {
        Board::try_from("KsAc-4c").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_23() {
        Board::try_from("KsAc-4h9").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_24() {
        Board::try_from("KsAc-4").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_25() {
        Board::try_from("ksAc4h").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_26() {
        Board::try_from("Kscc-4h").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_27() {
        Board::try_from("Ks").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_28() {
        Board::try_from("").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_29() {
        Board::try_from(".").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_30() {
        Board::try_from("------").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_try_from_31() {
        Board::try_from("KsAcAc").unwrap();
    }
}
