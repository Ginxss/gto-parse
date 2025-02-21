use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Betsize {
    Size33,
    Size50,
    Size75,
    Size150,
}

impl Display for Betsize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Betsize::Size33 => "33",
            Betsize::Size50 => "50",
            Betsize::Size75 => "75",
            Betsize::Size150 => "150",
        };

        return write!(f, "{}", s);
    }
}

impl FromStr for Betsize {
    type Err = InvalidBetsize;

    fn from_str(s: &str) -> Result<Betsize, InvalidBetsize> {
        match s {
            "33" => Ok(Betsize::Size33),
            "50" => Ok(Betsize::Size50),
            "75" => Ok(Betsize::Size75),
            "150" => Ok(Betsize::Size150),
            other => Err(InvalidBetsize(other.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct InvalidBetsize(String);

impl Display for InvalidBetsize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid betsize: {}", self.0)
    }
}

impl Error for InvalidBetsize {}
