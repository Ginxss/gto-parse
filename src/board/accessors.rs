use crate::board::validators::{is_valid_card, is_valid_flop, is_valid_rank, is_valid_suit};

pub fn get_rank(flop: &str, i: usize) -> char {
    assert!(is_valid_flop(flop));
    assert!((0..=2).contains(&i));

    let rank = flop.chars().nth(i * 2).unwrap();

    assert!(is_valid_rank(&rank));

    rank
}

pub fn get_suit(flop: &str, i: usize) -> char {
    assert!(is_valid_flop(flop));
    assert!((0..=2).contains(&i));

    let suit = flop.chars().nth(i * 2 + 1).unwrap();

    assert!(is_valid_suit(&suit));

    suit
}

pub fn get_card(flop: &str, i: usize) -> &str {
    assert!(is_valid_flop(flop));
    assert!((0..=2).contains(&i));

    let card = &flop[i * 2..(i + 1) * 2];

    assert!(is_valid_card(card));

    card
}

pub fn get_ranks(flop: &str) -> Vec<char> {
    assert!(is_valid_flop(flop));

    let ranks: Vec<char> = flop
        .char_indices()
        .filter_map(|(i, c)| (i % 2 == 0).then(|| c))
        .collect();

    assert!(ranks.len() == 3);
    assert!(ranks.iter().all(|rank| is_valid_rank(rank)));

    ranks
}

pub fn get_suits(flop: &str) -> Vec<char> {
    assert!(is_valid_flop(flop));

    let suits: Vec<char> = flop
        .char_indices()
        .filter_map(|(i, c)| (i % 2 != 0).then(|| c))
        .collect();

    assert!(suits.len() == 3);
    assert!(suits.iter().all(|suit| is_valid_suit(suit)));

    suits
}

pub fn get_cards(flop: &str) -> Vec<&str> {
    assert!(is_valid_flop(flop));

    let cards = vec![&flop[..2], &flop[2..4], &flop[4..]];

    assert!(cards.len() == 3);
    assert!(cards.iter().all(|card| is_valid_card(card)));

    cards
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
    #[should_panic]
    fn test_get_rank_invalid_board() {
        get_rank("KsAc77", 1);
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
    #[should_panic]
    fn test_get_suit_invalid_board() {
        get_suit("KsAc77", 1);
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
    #[should_panic]
    fn test_get_card_invalid_board() {
        get_card("KsAc77", 1);
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
    #[should_panic]
    fn test_get_ranks_board_too_short() {
        get_ranks("KsAc7");
    }

    #[test]
    #[should_panic]
    fn test_get_ranks_invalid_board() {
        get_ranks("ksAc7h");
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
    #[should_panic]
    fn test_get_suits_board_too_short() {
        get_suits("KsAc7");
    }

    #[test]
    #[should_panic]
    fn test_get_suits_invalid_board() {
        get_suits("Kscc7h");
    }

    #[test]
    fn test_get_cards() {
        assert_eq!(get_cards("KsAc7h"), vec!["Ks", "Ac", "7h"]);
    }

    #[test]
    #[should_panic]
    fn test_get_cards_board_too_long() {
        get_cards("KsAc7h9c");
    }

    #[test]
    #[should_panic]
    fn test_get_cards_board_too_short() {
        get_suits("Ks");
    }

    #[test]
    #[should_panic]
    fn test_get_cards_invalid_board() {
        get_suits("KSAc7h");
    }
}
