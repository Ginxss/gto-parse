use std::collections::HashMap;

use crate::board::util::{accessors::get_suits, validators::is_valid_flop};

fn get_max_suit_count(flop: &str) -> usize {
    assert!(is_valid_flop(flop));

    let max_suit_count = *get_suits(flop)
        .iter()
        .fold(HashMap::new(), |mut acc, suit| {
            *acc.entry(suit).or_insert(0) += 1;
            acc
        })
        .values()
        .max()
        .unwrap();

    assert!(max_suit_count <= 3);

    max_suit_count
}

pub fn is_rainbow(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    get_max_suit_count(flop) == 1
}

pub fn is_twotone(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    get_max_suit_count(flop) == 2
}

pub fn is_monotone(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    get_max_suit_count(flop) == 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_suit_count() {
        assert_eq!(get_max_suit_count("Qc9h7h"), 2);
        assert_eq!(get_max_suit_count("Qc9s7h"), 1);
        assert_eq!(get_max_suit_count("Qc9c7c"), 3);
    }

    #[test]
    #[should_panic]
    fn test_get_max_suit_count_invalid_flop() {
        get_max_suit_count("0c6h5s");
    }

    #[test]
    fn test_is_rainbow() {
        assert!(is_rainbow("6s5h4c"));
        assert!(is_rainbow("6s5d4h"));
        assert!(!is_rainbow("5c3h2h"));
        assert!(!is_rainbow("Ah2h3h"));
    }

    #[test]
    #[should_panic]
    fn test_is_rainbow_invalid_flop() {
        is_rainbow("1c6h5s");
    }

    #[test]
    fn test_is_twotone() {
        assert!(is_twotone("6s5s4c"));
        assert!(is_twotone("6s5d4d"));
        assert!(!is_twotone("5c3h2s"));
        assert!(!is_twotone("Ac2c3c"));
    }

    #[test]
    #[should_panic]
    fn test_is_twotone_invalid_flop() {
        is_twotone("216h5s");
    }

    #[test]
    fn test_is_monotone() {
        assert!(is_monotone("6h5h4h"));
        assert!(is_monotone("5c3c2c"));
        assert!(!is_monotone("Ac2h3s"));
        assert!(!is_monotone("Tc8sKs"));
    }

    #[test]
    #[should_panic]
    fn test_is_monotone_invalid_flop() {
        is_monotone("10c6h5s");
    }
}
