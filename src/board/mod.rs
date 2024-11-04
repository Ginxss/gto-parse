mod util;

pub mod connectedness;
pub mod flop_height;
pub mod suits;

/*
 * Terminology:
 * Rank = card without suit (e.g. A)
 * Suit = suit modifier after rank (s, c, d or h)
 * Card = rank + suit (e.g. As)
 * Value = card rank as numerical value (A is 12, 2 is 0)
 * RankCategory = broadway, middling or low
 */
