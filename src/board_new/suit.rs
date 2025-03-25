use super::BoardParseError;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Suit {
    Spade,
    Club,
    Diamond,
    Heart,
}

impl TryFrom<char> for Suit {
    type Error = BoardParseError;

    fn try_from(c: char) -> Result<Suit, BoardParseError> {
        match c {
            's' => Ok(Suit::Spade),
            'c' => Ok(Suit::Club),
            'd' => Ok(Suit::Diamond),
            'h' => Ok(Suit::Heart),
            _ => Err(BoardParseError::char("suit", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suit_from() {
        todo!();
    }
}
