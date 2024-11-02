use core::panic;
use std::collections::HashMap;

mod validators;

/*
 * Terminology:
 * Rank = card without suit (e.g. A)
 * Suit = suit modifier after rank (s, c, d or h)
 * Card = rank + suit (e.g. As)
 * Value = card rank as numerical value (A is 12, 2 is 0)
 * CardCategory = broadway, middling or low
 */

#[derive(Debug, PartialEq)]
enum CardCategory {
    Broadway,
    Middling,
    Low,
}

fn get_rank(flop: &str, i: usize) -> char {
    assert!(validators::is_valid_flop(flop));
    assert!((0..=2).contains(&i));

    let rank = flop.chars().nth(i * 2).unwrap();

    assert!(validators::is_valid_rank(&rank));

    rank
}

fn get_suit(flop: &str, i: usize) -> char {
    assert!(validators::is_valid_flop(flop));
    assert!((0..=2).contains(&i));

    let suit = flop.chars().nth(i * 2 + 1).unwrap();

    assert!(validators::is_valid_suit(&suit));

    suit
}

fn get_card(flop: &str, i: usize) -> &str {
    assert!(validators::is_valid_flop(flop));
    assert!((0..=2).contains(&i));

    let card = &flop[i * 2..(i + 1) * 2];

    assert!(validators::is_valid_card(card));

    card
}

fn get_ranks(flop: &str) -> Vec<char> {
    assert!(validators::is_valid_flop(flop));

    let ranks: Vec<char> = flop
        .char_indices()
        .filter_map(|(i, c)| (i % 2 == 0).then(|| c))
        .collect();

    assert!(ranks.len() == 3);
    assert!(ranks.iter().all(|rank| validators::is_valid_rank(rank)));

    ranks
}

fn get_suits(flop: &str) -> Vec<char> {
    assert!(validators::is_valid_flop(flop));

    let suits: Vec<char> = flop
        .char_indices()
        .filter_map(|(i, c)| (i % 2 != 0).then(|| c))
        .collect();

    assert!(suits.len() == 3);

    suits
}

fn get_cards(flop: &str) -> Vec<&str> {
    assert!(validators::is_valid_flop(flop));

    let cards = vec![&flop[..2], &flop[2..4], &flop[4..]];

    assert!(cards.len() == 3);
    assert!(cards[0].len() == 2);
    assert!(cards[1].len() == 2);
    assert!(cards[2].len() == 2);

    cards
}

fn get_category(rank: &char) -> CardCategory {
    match rank {
        '2' | '3' | '4' | '5' | '6' => CardCategory::Low,
        '9' | '8' | '7' => CardCategory::Middling,
        'T' | 'J' | 'Q' | 'K' | 'A' => CardCategory::Broadway,
        _ => panic!("Invalid rank"),
    }
}

fn get_value(rank: &char) -> i8 {
    match rank {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Invalid rank"),
    }
}

fn num_card_category(flop: &str, filter_category: CardCategory) -> usize {
    get_ranks(flop)
        .iter()
        .map(|rank| get_category(&rank))
        .filter(|category| *category == filter_category)
        .count()
}

pub fn is_1bw(flop: &str) -> bool {
    num_card_category(flop, CardCategory::Broadway) == 1
}

pub fn is_2bw(flop: &str) -> bool {
    num_card_category(flop, CardCategory::Broadway) == 2
}

pub fn is_3bw(flop: &str) -> bool {
    num_card_category(flop, CardCategory::Broadway) == 3
}

pub fn is_middling(flop: &str) -> bool {
    num_card_category(flop, CardCategory::Broadway) == 0
        && num_card_category(flop, CardCategory::Middling) > 0
}

pub fn is_low(flop: &str) -> bool {
    num_card_category(flop, CardCategory::Broadway) == 0
        && num_card_category(flop, CardCategory::Middling) == 0
}

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

    let ranks = get_ranks(flop);

    let normal_straight_possible = get_diffs(&ranks).iter().sum::<i8>() <= 4;

    let wheel_straight_possible = ranks.iter().all(|rank| validators::is_wheel_rank(rank));

    normal_straight_possible || wheel_straight_possible
}

pub fn is_oesd_possible(flop: &str) -> bool {
    assert!(validators::is_valid_flop(flop));

    let ranks = get_ranks(flop);

    let diffs = get_diffs(&ranks);

    let no_straight_possible = diffs.iter().sum::<i8>() > 4;
    let oesd_possible = no_straight_possible && diffs.iter().any(|diff| *diff == 2 || *diff == 3);

    let a_draw = ranks.iter().any(|rank| *rank == 'A')
        && num_card_category(flop, CardCategory::Broadway) == 2;

    oesd_possible && !a_draw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rank() {
        let flop = "KsAc7c";
        assert_eq!(get_rank(flop, 0), 'K');
        assert_eq!(get_rank(flop, 1), 'A');
        assert_eq!(get_rank(flop, 2), '7');
    }

    #[test]
    #[should_panic]
    fn test_get_rank_invalid_idx() {
        get_rank("KsAc7c", 3);
    }

    #[test]
    #[should_panic]
    fn test_get_rank_board_too_long() {
        get_rank("KsAc7c6h", 1);
    }

    #[test]
    #[should_panic]
    fn test_get_rank_board_too_short() {
        get_rank("KsAc", 1);
    }

    #[test]
    fn test_get_suit() {
        let flop = "KsAc7h";
        assert_eq!(get_suit(flop, 0), 's');
        assert_eq!(get_suit(flop, 1), 'c');
        assert_eq!(get_suit(flop, 2), 'h');
    }

    #[test]
    #[should_panic]
    fn test_get_suit_invalid_idx() {
        get_suit("KsAc7c", 3);
    }

    #[test]
    #[should_panic]
    fn test_get_suit_board_too_long() {
        get_suit("KsAc7c6h", 1);
    }

    #[test]
    #[should_panic]
    fn test_get_suit_board_too_short() {
        get_suit("KsAc", 1);
    }

    #[test]
    fn test_get_card() {
        let flop = "KsAc7h";
        assert_eq!(get_card(flop, 0), "Ks");
        assert_eq!(get_card(flop, 1), "Ac");
        assert_eq!(get_card(flop, 2), "7h");
    }

    #[test]
    #[should_panic]
    fn test_get_card_invalid_idx() {
        get_card("KsAc7c", 3);
    }

    #[test]
    #[should_panic]
    fn test_get_card_board_too_long() {
        get_card("KsAc7c6h", 1);
    }

    #[test]
    #[should_panic]
    fn test_get_card_board_too_short() {
        get_card("KsAc", 1);
    }

    #[test]
    fn test_get_ranks() {
        assert_eq!(get_ranks("KsAc7h"), vec!['K', 'A', '7']);
    }

    #[test]
    #[should_panic]
    fn test_get_ranks_board_too_long() {
        get_ranks("KsAc7h9");
    }

    #[test]
    fn test_get_suits() {
        assert_eq!(get_suits("KsAc7h"), vec!['s', 'c', 'h']);
    }

    #[test]
    #[should_panic]
    fn test_get_suits_board_too_long() {
        get_suits("KsAc7h9");
    }

    #[test]
    fn test_get_cards() {
        assert_eq!(get_cards("KsAc7h"), vec!["Ks", "Ac", "7h"]);
    }

    #[test]
    #[should_panic]
    fn test_get_cards_board_too_long() {
        get_cards("KsAc7h9");
    }

    #[test]
    fn test_get_category() {
        assert_eq!(get_category(&'A'), CardCategory::Broadway);
        assert_eq!(get_category(&'K'), CardCategory::Broadway);
        assert_eq!(get_category(&'Q'), CardCategory::Broadway);
        assert_eq!(get_category(&'J'), CardCategory::Broadway);
        assert_eq!(get_category(&'T'), CardCategory::Broadway);
        assert_eq!(get_category(&'9'), CardCategory::Middling);
        assert_eq!(get_category(&'8'), CardCategory::Middling);
        assert_eq!(get_category(&'7'), CardCategory::Middling);
        assert_eq!(get_category(&'6'), CardCategory::Low);
        assert_eq!(get_category(&'5'), CardCategory::Low);
        assert_eq!(get_category(&'4'), CardCategory::Low);
        assert_eq!(get_category(&'3'), CardCategory::Low);
        assert_eq!(get_category(&'2'), CardCategory::Low);
    }

    #[test]
    #[should_panic]
    fn test_get_category_invalid() {
        get_category(&'a');
    }

    #[test]
    fn test_get_value() {
        assert_eq!(get_value(&'A'), 12);
        assert_eq!(get_value(&'K'), 11);
        assert_eq!(get_value(&'Q'), 10);
        assert_eq!(get_value(&'J'), 9);
        assert_eq!(get_value(&'T'), 8);
        assert_eq!(get_value(&'9'), 7);
        assert_eq!(get_value(&'8'), 6);
        assert_eq!(get_value(&'7'), 5);
        assert_eq!(get_value(&'6'), 4);
        assert_eq!(get_value(&'5'), 3);
        assert_eq!(get_value(&'4'), 2);
        assert_eq!(get_value(&'3'), 1);
        assert_eq!(get_value(&'2'), 0);
    }

    #[test]
    fn test_num_card_category() {
        let flop = "3h9cTh";
        assert_eq!(num_card_category(flop, CardCategory::Broadway), 1);
        assert_eq!(num_card_category(flop, CardCategory::Middling), 1);
        assert_eq!(num_card_category(flop, CardCategory::Low), 1);

        let flop = "AsAc6h";
        assert_eq!(num_card_category(flop, CardCategory::Broadway), 2);
        assert_eq!(num_card_category(flop, CardCategory::Middling), 0);
        assert_eq!(num_card_category(flop, CardCategory::Low), 1);

        let flop = "Qc9h7h";
        assert_eq!(num_card_category(flop, CardCategory::Broadway), 1);
        assert_eq!(num_card_category(flop, CardCategory::Middling), 2);
        assert_eq!(num_card_category(flop, CardCategory::Low), 0);
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
