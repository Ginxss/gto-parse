use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use super::ParseError;

const CHECK_STR: &str = "X";
const BET_STR: &str = "B";
const CALL_STR: &str = "C";
const RAISE_STR: &str = "R";
const FOLD_STR: &str = "F";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Check,
    Bet,
    Call,
    Raise,
    Fold,
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Action, ParseError> {
        match s {
            CHECK_STR => Ok(Action::Check),
            BET_STR => Ok(Action::Bet),
            CALL_STR => Ok(Action::Call),
            RAISE_STR => Ok(Action::Raise),
            FOLD_STR => Ok(Action::Fold),
            _ => Err(ParseError::str("action", s)),
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Action::Check => CHECK_STR,
            Action::Bet => BET_STR,
            Action::Call => CALL_STR,
            Action::Raise => RAISE_STR,
            Action::Fold => FOLD_STR,
        };

        return write!(f, "{}", s);
    }
}

impl Action {
    pub fn to_long_string(&self) -> String {
        let s = match self {
            Action::Check => "check",
            Action::Bet => "bet",
            Action::Call => "call",
            Action::Raise => "raise",
            Action::Fold => "fold",
        };

        s.to_string()
    }
}
