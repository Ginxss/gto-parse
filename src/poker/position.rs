use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use super::ParseError;

const LJ_STR: &str = "LJ";
const HJ_STR: &str = "HJ";
const CO_STR: &str = "CO";
const BTN_STR: &str = "BTN";
const SB_STR: &str = "SB";
const BB_STR: &str = "BB";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Position {
    LJ,
    HJ,
    CO,
    BTN,
    SB,
    BB,
}

impl FromStr for Position {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Position, ParseError> {
        match s {
            LJ_STR => Ok(Position::LJ),
            HJ_STR => Ok(Position::HJ),
            CO_STR => Ok(Position::CO),
            BTN_STR => Ok(Position::BTN),
            SB_STR => Ok(Position::SB),
            BB_STR => Ok(Position::BB),
            _ => Err(ParseError::str("position", s)),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Position::LJ => LJ_STR,
            Position::HJ => HJ_STR,
            Position::CO => CO_STR,
            Position::BTN => BTN_STR,
            Position::SB => SB_STR,
            Position::BB => BB_STR,
        };

        return write!(f, "{}", s);
    }
}

pub struct Positions {
    pub ip: Position,
    pub oop: Position,
}
