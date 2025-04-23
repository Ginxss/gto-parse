use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    ops::Sub,
};

use super::{
    rank::{Rank, RankHeight},
    suit::Suit,
    ParseError,
};

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl TryFrom<&str> for Card {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Card, ParseError> {
        if s.len() != 2 {
            return Err(ParseError::str("card", s));
        }

        let mut chars = s.chars();
        let rank = chars.next().unwrap();
        let suit = chars.next().unwrap();

        Card::try_from((rank, suit))
    }
}

impl TryFrom<(char, char)> for Card {
    type Error = ParseError;

    fn try_from((rank, suit): (char, char)) -> Result<Card, ParseError> {
        let rank = Rank::try_from(rank)?;
        let suit = Suit::try_from(suit)?;

        Ok(Card { rank, suit })
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
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
    fn test_card_try_from_str() {
        let card = Card::try_from("As").unwrap();
        assert_eq!(card.rank, Rank::A);
        assert_eq!(card.suit, Suit::Spade);
    }

    #[test]
    fn test_card_try_from_chars() {
        let card = Card::try_from(('K', 'c')).unwrap();
        assert_eq!(card.rank, Rank::K);
        assert_eq!(card.suit, Suit::Club);
    }

    #[test]
    fn test_card_is_bw() {
        assert!(Card::try_from("Ac").unwrap().is_bw());
        assert!(Card::try_from("Qc").unwrap().is_bw());
        assert!(!Card::try_from("9c").unwrap().is_bw());
        assert!(!Card::try_from("7c").unwrap().is_bw());
        assert!(!Card::try_from("5c").unwrap().is_bw());
        assert!(!Card::try_from("2c").unwrap().is_bw());
    }

    #[test]
    fn test_card_is_middling() {
        assert!(!Card::try_from("Ac").unwrap().is_middling());
        assert!(!Card::try_from("Qc").unwrap().is_middling());
        assert!(Card::try_from("9c").unwrap().is_middling());
        assert!(Card::try_from("7c").unwrap().is_middling());
        assert!(!Card::try_from("5c").unwrap().is_middling());
        assert!(!Card::try_from("2c").unwrap().is_middling());
    }

    #[test]
    fn test_card_is_low() {
        assert!(!Card::try_from("Ac").unwrap().is_low());
        assert!(!Card::try_from("Qc").unwrap().is_low());
        assert!(!Card::try_from("9c").unwrap().is_low());
        assert!(!Card::try_from("7c").unwrap().is_low());
        assert!(Card::try_from("5c").unwrap().is_low());
        assert!(Card::try_from("2c").unwrap().is_low());
    }

    #[test]
    fn test_card_is_wheel() {
        assert!(Card::try_from("Ac").unwrap().is_wheel());
        assert!(!Card::try_from("Qc").unwrap().is_wheel());
        assert!(!Card::try_from("9c").unwrap().is_wheel());
        assert!(!Card::try_from("7c").unwrap().is_wheel());
        assert!(Card::try_from("5c").unwrap().is_wheel());
        assert!(Card::try_from("2c").unwrap().is_wheel());
    }

    #[test]
    fn test_card_is_height() {
        assert!(Card::try_from("Ah")
            .unwrap()
            .is_height(&RankHeight::Broadway));
        assert!(!Card::try_from("Ah")
            .unwrap()
            .is_height(&RankHeight::Middling));
        assert!(!Card::try_from("Ah").unwrap().is_height(&RankHeight::Low));
        assert!(Card::try_from("Ah").unwrap().is_height(&RankHeight::Wheel));

        assert!(!Card::try_from("9c")
            .unwrap()
            .is_height(&RankHeight::Broadway));
        assert!(Card::try_from("9h")
            .unwrap()
            .is_height(&RankHeight::Middling));
        assert!(!Card::try_from("9h").unwrap().is_height(&RankHeight::Low));
        assert!(!Card::try_from("9h").unwrap().is_height(&RankHeight::Wheel));

        assert!(!Card::try_from("6c")
            .unwrap()
            .is_height(&RankHeight::Broadway));
        assert!(!Card::try_from("6h")
            .unwrap()
            .is_height(&RankHeight::Middling));
        assert!(Card::try_from("6h").unwrap().is_height(&RankHeight::Low));
        assert!(!Card::try_from("6h").unwrap().is_height(&RankHeight::Wheel));

        assert!(!Card::try_from("3c")
            .unwrap()
            .is_height(&RankHeight::Broadway));
        assert!(!Card::try_from("3h")
            .unwrap()
            .is_height(&RankHeight::Middling));
        assert!(Card::try_from("3h").unwrap().is_height(&RankHeight::Low));
        assert!(Card::try_from("3h").unwrap().is_height(&RankHeight::Wheel));
    }

    #[test]
    fn test_card_is_ace() {
        assert!(Card::try_from("Ah").unwrap().is_ace());
        assert!(!Card::try_from("9h").unwrap().is_ace());
        assert!(!Card::try_from("6h").unwrap().is_ace());
        assert!(!Card::try_from("2h").unwrap().is_ace());
    }

    #[test]
    fn test_get_distances() {
        let cards = vec![
            Card::try_from("Qc").unwrap(),
            Card::try_from("Th").unwrap(),
            Card::try_from("7c").unwrap(),
            Card::try_from(('7', 'h')).unwrap(),
        ];

        assert_eq!(Card::get_distances(cards.iter()), vec![2, 3, 0]);
    }

    #[test]
    fn test_card_cmp_greater() {
        let card1 = Card::try_from("Ks").unwrap();
        let card2 = Card::try_from("8c").unwrap();

        assert_eq!(card1.cmp(&card2), Ordering::Greater);
        assert_eq!(card1.partial_cmp(&card2), Some(Ordering::Greater));

        assert_eq!(card1 > card2, true);
        assert_eq!(card1 == card2, false);
        assert_eq!(card1 < card2, false);
    }

    #[test]
    fn test_card_cmp_less() {
        let card1 = Card::try_from("3h").unwrap();
        let card2 = Card::try_from("4h").unwrap();

        assert_eq!(card1.cmp(&card2), Ordering::Less);
        assert_eq!(card1.partial_cmp(&card2), Some(Ordering::Less));

        assert_eq!(card1 > card2, false);
        assert_eq!(card1 == card2, false);
        assert_eq!(card1 < card2, true);
    }

    #[test]
    fn test_card_cmp_equal_1() {
        let card1 = Card::try_from("3h").unwrap();
        let card2 = Card::try_from("3c").unwrap();

        assert_eq!(card1.cmp(&card2), Ordering::Equal);
        assert_eq!(card1.partial_cmp(&card2), Some(Ordering::Equal));

        assert_eq!(card1 > card2, false);
        assert_eq!(card1 == card2, false);
        assert_eq!(card1 < card2, false);
    }

    #[test]
    fn test_card_cmp_equal_2() {
        let card1 = Card::try_from("3h").unwrap();
        let card2 = Card::try_from("3h").unwrap();

        assert_eq!(card1.cmp(&card2), Ordering::Equal);
        assert_eq!(card1.partial_cmp(&card2), Some(Ordering::Equal));

        assert_eq!(card1 > card2, false);
        assert_eq!(card1 == card2, true);
        assert_eq!(card1 < card2, false);
    }

    #[test]
    fn test_card_sub_1() {
        let card1 = Card::try_from("3h").unwrap();
        let card2 = Card::try_from("3c").unwrap();
        assert_eq!(&card1 - &card2, 0);
        assert_eq!(card1 - card2, 0);
    }

    #[test]
    fn test_card_sub_2() {
        let card1 = Card::try_from("8c").unwrap();
        let card2 = Card::try_from("4c").unwrap();
        assert_eq!(&card1 - &card2, 4);
        assert_eq!(card1 - card2, 4);
    }

    #[test]
    fn test_card_sub_3() {
        let card1 = Card::try_from("Th").unwrap();
        let card2 = Card::try_from("Ac").unwrap();
        assert_eq!(&card1 - &card2, -4);
        assert_eq!(card1 - card2, -4);
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_1() {
        Card::try_from("AA").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_2() {
        Card::try_from("A").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_3() {
        Card::try_from("__").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_4() {
        Card::try_from("dc").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_5() {
        Card::try_from("7cc").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_6() {
        Card::try_from("8c4").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_7() {
        Card::try_from("2h5c").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_8() {
        Card::try_from(" Js").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_9() {
        Card::try_from("Wd").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_10() {
        Card::try_from("Th ").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_11() {
        Card::try_from("TD").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_12() {
        Card::try_from("Qa").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_13() {
        Card::try_from("QcT").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_14() {
        Card::try_from("hh").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_str_15() {
        Card::try_from("..").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_chars_1() {
        Card::try_from(('c', 'K')).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_chars_2() {
        Card::try_from(('T', 'H')).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_chars_3() {
        Card::try_from(('ö', 'c')).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_chars_4() {
        Card::try_from(('.', 'c')).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_chars_5() {
        Card::try_from(('2', '2')).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_chars_6() {
        Card::try_from((' ', ' ')).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_try_from_invalid_chars_7() {
        Card::try_from(('u', 's')).unwrap();
    }
}
