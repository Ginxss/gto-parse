/*
 * Terminology:
 * Rank = card without suit (e.g. A)
 * Suit = suit modifier after rank (s, c, d or h)
 * Card = rank + suit (e.g. As)
 * Board = list of cards (e.g. flop = 3 cards)
 */

pub mod action;
pub mod betsize;
pub mod board;
pub mod card;
pub mod position;
pub mod rank;
pub mod suit;

use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct ParseError(String);

impl ParseError {
    fn char(obj: &str, src: char) -> ParseError {
        ParseError::str(obj, &src.to_string())
    }

    fn str(obj: &str, src: &str) -> ParseError {
        ParseError(format!("error parsing {obj} from {src}"))
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_parse_error_1() {
        let err = ParseError::char("object", 'a');
        assert_eq!(err.0, "error parsing object from a")
    }

    #[test]
    fn test_board_parse_error_2() {
        let err = ParseError::str("objects", "aaa");
        assert_eq!(err.0, "error parsing objects from aaa")
    }
}
