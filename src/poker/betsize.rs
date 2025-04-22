use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use super::ParseError;

const SIZE33_STR: &str = "33";
const SIZE50_STR: &str = "50";
const SIZE75_STR: &str = "75";
const SIZE150_STR: &str = "150";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Betsize {
    Size33,
    Size50,
    Size75,
    Size150,
}

impl FromStr for Betsize {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Betsize, ParseError> {
        match s {
            SIZE33_STR => Ok(Betsize::Size33),
            SIZE50_STR => Ok(Betsize::Size50),
            SIZE75_STR => Ok(Betsize::Size75),
            SIZE150_STR => Ok(Betsize::Size150),
            _ => Err(ParseError::str("betsize", s)),
        }
    }
}

impl Display for Betsize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Betsize::Size33 => SIZE33_STR,
            Betsize::Size50 => SIZE50_STR,
            Betsize::Size75 => SIZE75_STR,
            Betsize::Size150 => SIZE150_STR,
        };

        return write!(f, "{}", s);
    }
}
