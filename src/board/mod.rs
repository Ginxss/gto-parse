mod accessors;
mod rank_properties;
mod validators;

use accessors::{get_ranks, get_suits};
use rank_properties::{get_category, get_value, RankCategory};
use std::collections::HashMap;

/*
 * Terminology:
 * Rank = card without suit (e.g. A)
 * Suit = suit modifier after rank (s, c, d or h)
 * Card = rank + suit (e.g. As)
 * Value = card rank as numerical value (A is 12, 2 is 0)
 * RankCategory = broadway, middling or low
 */

/// Flop Category ///

fn num_card_category(flop: &str, filter_category: RankCategory) -> usize {
    get_ranks(flop)
        .iter()
        .map(|rank| get_category(&rank))
        .filter(|category| *category == filter_category)
        .count()
}

pub fn is_1bw(flop: &str) -> bool {
    num_card_category(flop, RankCategory::Broadway) == 1
}

pub fn is_2bw(flop: &str) -> bool {
    num_card_category(flop, RankCategory::Broadway) == 2
}

pub fn is_3bw(flop: &str) -> bool {
    num_card_category(flop, RankCategory::Broadway) == 3
}

pub fn is_middling(flop: &str) -> bool {
    num_card_category(flop, RankCategory::Broadway) == 0
        && num_card_category(flop, RankCategory::Middling) > 0
}

pub fn is_low(flop: &str) -> bool {
    num_card_category(flop, RankCategory::Broadway) == 0
        && num_card_category(flop, RankCategory::Middling) == 0
}

/// Suits ///

fn get_max_suit_count(flop: &str) -> i32 {
    *get_suits(flop)
        .iter()
        .fold(HashMap::new(), |mut acc, suit| {
            *acc.entry(suit).or_insert(0) += 1;
            acc
        })
        .values()
        .max()
        .expect("Expected max to be there")
}

pub fn is_rainbow(flop: &str) -> bool {
    get_max_suit_count(flop) == 1
}

pub fn is_twotone(flop: &str) -> bool {
    get_max_suit_count(flop) == 2
}

pub fn is_monotone(flop: &str) -> bool {
    get_max_suit_count(flop) == 3
}

/// Connectedness ///

fn get_diffs(ranks: &Vec<char>) -> Vec<i8> {
    assert!(ranks.len() == 3);
    assert!(ranks.iter().all(|rank| validators::is_valid_rank(rank)));

    let diffs: Vec<i8> = ranks
        .windows(2)
        .map(|ranks| get_value(&ranks[0]) - get_value(&ranks[1]))
        .map(|diff| diff.abs())
        .collect();

    assert!(diffs.len() == 2);

    diffs
}

pub fn is_straight_possible(flop: &str) -> bool {
    assert!(validators::is_valid_flop(flop));

    let ranks = accessors::get_ranks(flop);

    let normal_straight_possible = get_diffs(&ranks).iter().sum::<i8>() <= 4;

    let wheel_straight_possible = ranks.iter().all(|rank| validators::is_wheel_rank(rank));

    normal_straight_possible || wheel_straight_possible
}

pub fn is_oesd_possible(flop: &str) -> bool {
    assert!(validators::is_valid_flop(flop));

    let ranks = accessors::get_ranks(flop);

    let diffs = get_diffs(&ranks);

    let no_straight_possible = diffs.iter().sum::<i8>() > 4;
    let oesd_possible = no_straight_possible && diffs.iter().any(|diff| *diff == 2 || *diff == 3);

    let a_draw = ranks.iter().any(|rank| *rank == 'A')
        && num_card_category(flop, RankCategory::Broadway) == 2;

    oesd_possible && !a_draw
}

/// Tests ///

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_card_category() {
        let flop = "3h9cTh";
        assert_eq!(num_card_category(flop, RankCategory::Broadway), 1);
        assert_eq!(num_card_category(flop, RankCategory::Middling), 1);
        assert_eq!(num_card_category(flop, RankCategory::Low), 1);

        let flop = "AsAc6h";
        assert_eq!(num_card_category(flop, RankCategory::Broadway), 2);
        assert_eq!(num_card_category(flop, RankCategory::Middling), 0);
        assert_eq!(num_card_category(flop, RankCategory::Low), 1);

        let flop = "Qc9h7h";
        assert_eq!(num_card_category(flop, RankCategory::Broadway), 1);
        assert_eq!(num_card_category(flop, RankCategory::Middling), 2);
        assert_eq!(num_card_category(flop, RankCategory::Low), 0);
    }

    #[test]
    fn test_is_1bw() {
        assert!(is_1bw("Ac9h6s"));
        assert!(is_1bw("Tc9h6s"));
        assert!(!is_1bw("9c8h6s"));
        assert!(!is_1bw("Kc4hQs"));
    }

    #[test]
    fn test_is_2bw() {
        assert!(is_2bw("Kc4hQs"));
        assert!(is_2bw("AcTh6s"));
        assert!(!is_2bw("Tc9h6s"));
        assert!(!is_2bw("9c2h6s"));
    }

    #[test]
    fn test_is_3bw() {
        assert!(is_3bw("KcJhQs"));
        assert!(is_3bw("AcKhTs"));
        assert!(!is_3bw("AcKh6s"));
        assert!(!is_3bw("Tc8s2s"));
    }

    #[test]
    fn test_is_middling() {
        assert!(is_middling("9h3c2c"));
        assert!(is_middling("8h7c2c"));
        assert!(!is_middling("9h3cAc"));
        assert!(!is_middling("6h3c2c"));
    }

    #[test]
    fn test_is_low() {
        assert!(is_low("6h5h4h"));
        assert!(is_low("5c3h2s"));
        assert!(!is_low("Ac2h3s"));
        assert!(!is_low("Tc8sKs"));
    }

    #[test]
    fn test_get_max_suit_count() {
        assert_eq!(get_max_suit_count("Qc9h7h"), 2);
        assert_eq!(get_max_suit_count("Qc9s7h"), 1);
        assert_eq!(get_max_suit_count("Qc9c7c"), 3);
    }

    #[test]
    fn test_is_rainbow() {
        assert!(is_rainbow("6s5h4c"));
        assert!(is_rainbow("6s5d4h"));
        assert!(!is_rainbow("5c3h2h"));
        assert!(!is_rainbow("Ah2h3h"));
    }

    #[test]
    fn test_is_twotone() {
        assert!(is_twotone("6s5s4c"));
        assert!(is_twotone("6s5d4d"));
        assert!(!is_twotone("5c3h2s"));
        assert!(!is_twotone("Ac2c3c"));
    }

    #[test]
    fn test_is_monotone() {
        assert!(is_monotone("6h5h4h"));
        assert!(is_monotone("5c3c2c"));
        assert!(!is_monotone("Ac2h3s"));
        assert!(!is_monotone("Tc8sKs"));
    }

    #[test]
    fn test_is_straight_possible() {
        assert!(is_straight_possible("Ts9c8h"));
        assert!(is_straight_possible("Ts9c7h"));
        assert!(is_straight_possible("Ts9c6h"));
        assert!(!is_straight_possible("Ts9c5h"));
        assert!(is_straight_possible("As2c3h"));
        assert!(!is_straight_possible("Ks2c3h"));
    }
}
