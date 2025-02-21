use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

pub struct Positions {
    pub ip: Position,
    pub oop: Position,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Position {
    LJ,
    HJ,
    CO,
    BTN,
    SB,
    BB,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Position::LJ => "LJ",
            Position::HJ => "HJ",
            Position::CO => "CO",
            Position::BTN => "BTN",
            Position::SB => "SB",
            Position::BB => "BB",
        };

        return write!(f, "{}", s);
    }
}

impl FromStr for Position {
    type Err = InvalidPosition;

    fn from_str(s: &str) -> Result<Position, InvalidPosition> {
        match s {
            "LJ" => Ok(Position::LJ),
            "HJ" => Ok(Position::HJ),
            "CO" => Ok(Position::CO),
            "BTN" => Ok(Position::BTN),
            "SB" => Ok(Position::SB),
            "BB" => Ok(Position::BB),
            other => Err(InvalidPosition(other.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct InvalidPosition(String);

impl Display for InvalidPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid position: {}", self.0)
    }
}

impl Error for InvalidPosition {}
