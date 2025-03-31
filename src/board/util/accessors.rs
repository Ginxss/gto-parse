use crate::board::util::validators::{is_valid_card, is_valid_flop, is_valid_rank, is_valid_suit};

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
