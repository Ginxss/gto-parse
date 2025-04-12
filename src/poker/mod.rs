/*
 * Terminology:
 * Rank = card without suit (e.g. A)
 * Suit = suit modifier after rank (s, c, d or h)
 * Card = rank + suit (e.g. As)
 * Board = list of cards (e.g. flop = 3 cards)
 */

pub mod board;
pub mod card;
pub mod rank;
pub mod suit;

use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct BoardParseError(String);

impl BoardParseError {
    fn char(obj: &str, src: char) -> BoardParseError {
        BoardParseError::str(obj, &src.to_string())
    }

    fn str(obj: &str, src: &str) -> BoardParseError {
        BoardParseError(format!("error parsing {obj} from {src}"))
    }
}

impl Display for BoardParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for BoardParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_parse_error_1() {
        let err = BoardParseError::char("object", 'a');
        assert_eq!(err.0, "error parsing object from a")
    }

    #[test]
    fn test_board_parse_error_2() {
        let err = BoardParseError::str("objects", "aaa");
        assert_eq!(err.0, "error parsing objects from aaa")
    }
}
