use std::{cmp::Ordering, ops::Sub};

use super::{
    rank::{Rank, RankHeight},
    suit::Suit,
    BoardParseError,
};

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl TryFrom<&str> for Card {
    type Error = BoardParseError;

    fn try_from(s: &str) -> Result<Card, BoardParseError> {
        if s.len() != 2 {
            return Err(BoardParseError::str("card", s));
        }

        let mut chars = s.chars();
        let rank = chars.next().unwrap();
        let suit = chars.next().unwrap();

        Card::try_from((rank, suit))
    }
}

impl TryFrom<(char, char)> for Card {
    type Error = BoardParseError;

    fn try_from((rank, suit): (char, char)) -> Result<Card, BoardParseError> {
        let rank = Rank::try_from(rank)?;
        let suit = Suit::try_from(suit)?;

        Ok(Card { rank, suit })
    }
}

impl Card {
    pub fn is_bw(&self) -> bool {
        self.rank.is_bw()
    }

    pub fn is_middling(&self) -> bool {
        self.rank.is_middling()
    }

    pub fn is_low(&self) -> bool {
        self.rank.is_low()
    }

    pub fn is_wheel(&self) -> bool {
        self.rank.is_wheel()
    }

    pub fn is_height(&self, height: &RankHeight) -> bool {
        self.rank.is_height(height)
    }

    pub fn get_heights(&self) -> Vec<RankHeight> {
        self.rank.get_heights()
    }

    pub fn is_ace(&self) -> bool {
        self.rank == Rank::A
    }

    pub fn get_distances<'a>(cards: impl Iterator<Item = &'a Card>) -> Vec<i32> {
        cards
            .collect::<Vec<_>>()
            .windows(2)
            .map(|cards| cards[1] - cards[0])
            .map(|diff| diff.abs())
            .collect()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl Sub for Card {
    type Output = i32;

    fn sub(self, rhs: Card) -> i32 {
        &self - &rhs
    }
}

impl Sub for &Card {
    type Output = i32;

    fn sub(self, rhs: &Card) -> i32 {
        self.rank - rhs.rank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from() {
        todo!();
    }
}
