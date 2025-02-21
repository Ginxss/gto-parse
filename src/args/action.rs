use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Check,
    Bet,
    Call,
    Raise,
    Fold,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            // TODO:
            Action::Check => "check",
            Action::Bet => "B",
            Action::Call => "C",
            Action::Raise => "R",
            Action::Fold => "F",
        };

        return write!(f, "{}", s);
    }
}

impl FromStr for Action {
    type Err = InvalidAction;

    fn from_str(s: &str) -> Result<Action, InvalidAction> {
        match s {
            "X" => Ok(Action::Check),
            "B" => Ok(Action::Bet),
            "C" => Ok(Action::Call),
            "R" => Ok(Action::Raise),
            "F" => Ok(Action::Fold),
            other => Err(InvalidAction(other.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct InvalidAction(String);

impl Display for InvalidAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid action: {}", self.0)
    }
}

impl Error for InvalidAction {}
