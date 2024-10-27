use core::panic;
use std::collections::HashMap;

/*
 * Terminology:
 * Rank = card without suit (e.g. A)
 * Suit = suit modifier after card (s, c, d or h)
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

fn get_rank(board: &str, i: usize) -> char {
    assert!(board.len() == 6);
    assert!((0..=2).contains(&i));

    board.chars().nth(i * 2).unwrap()
}

fn get_suit(board: &str, i: usize) -> char {
    assert!(board.len() == 6);
    assert!((0..=2).contains(&i));

    board.chars().nth(i * 2 + 1).unwrap()
}

fn get_card(board: &str, i: usize) -> &str {
    assert!(board.len() == 6);
    assert!((0..=2).contains(&i));

    &board[i * 2..(i + 1) * 2]
}

// TODO: Performance
fn num_card_category(board: &str, filter_category: CardCategory) -> usize {
    (0..=2)
        .map(|i| get_rank(board, i))
        .map(|rank| get_category(&rank))
        .filter(|category| *category == filter_category)
        .count()
}

pub fn is_1bw(board: &str) -> bool {
    num_card_category(board, CardCategory::Broadway) == 1
}

pub fn is_2bw(board: &str) -> bool {
    num_card_category(board, CardCategory::Broadway) == 2
}

pub fn is_3bw(board: &str) -> bool {
    num_card_category(board, CardCategory::Broadway) == 3
}

pub fn is_middling(board: &str) -> bool {
    num_card_category(board, CardCategory::Broadway) == 0
        && num_card_category(board, CardCategory::Middling) > 0
}

pub fn is_low(board: &str) -> bool {
    num_card_category(board, CardCategory::Broadway) == 0
        && num_card_category(board, CardCategory::Middling) == 0
}

fn get_suit_count(board: &str) -> HashMap<char, i32> {
    (0..=2)
        .map(|i| get_suit(board, i))
        .fold(HashMap::new(), |mut acc, suit| {
            *acc.entry(suit).or_insert(0) += 1;
            acc
        })
}

pub fn is_rainbow(board: &str) -> bool {
    *get_suit_count(board)
        .values()
        .max()
        .expect("Expected max to be there")
        == 1
}

pub fn is_twotone(board: &str) -> bool {
    *get_suit_count(board)
        .values()
        .max()
        .expect("Expected max to be there")
        == 2
}

pub fn is_monotone(board: &str) -> bool {
    *get_suit_count(board)
        .values()
        .max()
        .expect("Expected max to be there")
        == 3
}

// TODO: Connectedness

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rank() {
        let board = "KsAc7c";
        assert_eq!(get_rank(board, 0), 'K');
        assert_eq!(get_rank(board, 1), 'A');
        assert_eq!(get_rank(board, 2), '7');
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
        let board = "KsAc7h";
        assert_eq!(get_suit(board, 0), 's');
        assert_eq!(get_suit(board, 1), 'c');
        assert_eq!(get_suit(board, 2), 'h');
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
        let board = "KsAc7h";
        assert_eq!(get_card(board, 0), "Ks");
        assert_eq!(get_card(board, 1), "Ac");
        assert_eq!(get_card(board, 2), "7h");
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
        let board = "3h9cTh";
        assert_eq!(num_card_category(board, CardCategory::Broadway), 1);
        assert_eq!(num_card_category(board, CardCategory::Middling), 1);
        assert_eq!(num_card_category(board, CardCategory::Low), 1);

        let board = "AsAc6h";
        assert_eq!(num_card_category(board, CardCategory::Broadway), 2);
        assert_eq!(num_card_category(board, CardCategory::Middling), 0);
        assert_eq!(num_card_category(board, CardCategory::Low), 1);

        let board = "Qc9h7h";
        assert_eq!(num_card_category(board, CardCategory::Broadway), 1);
        assert_eq!(num_card_category(board, CardCategory::Middling), 2);
        assert_eq!(num_card_category(board, CardCategory::Low), 0);
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
    fn test_get_suit_count() {
        let suit_count = get_suit_count("Qc9h7h");
        assert_eq!(suit_count.get(&'c'), Some(&1));
        assert_eq!(suit_count.get(&'h'), Some(&2));
        assert_eq!(suit_count.get(&'s'), None);
        assert_eq!(suit_count.get(&'d'), None);
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
}
