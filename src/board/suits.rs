use std::collections::HashMap;

use crate::board::util::{accessors::get_suits, validators::is_valid_flop};

#[derive(Debug, PartialEq)]
pub enum FlopSuitType {
    Rainbow,
    Twotone,
    Montone,
}

impl FlopSuitType {
    pub fn from_str(string: &str) -> FlopSuitType {
        match string {
            "R" => FlopSuitType::Rainbow,
            "T" => FlopSuitType::Twotone,
            "M" => FlopSuitType::Montone,
            other => panic!("Invalid flop suit type string: {}", other),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            FlopSuitType::Rainbow => "R",
            FlopSuitType::Twotone => "T",
            FlopSuitType::Montone => "M",
        }
    }
}

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

pub fn get_suit_type(flop: &str) -> FlopSuitType {
    assert!(is_valid_flop(flop));

    match get_max_suit_count(flop) {
        1 => FlopSuitType::Rainbow,
        2 => FlopSuitType::Twotone,
        3 => FlopSuitType::Montone,
        other => panic!("Invalid suit count: {}", other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_suit_count() {
        assert_eq!(get_max_suit_count("Qc9s7h"), 1);
        assert_eq!(get_max_suit_count("Qc9h7h"), 2);
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
        assert!(!is_rainbow("6c5h4h"));
        assert!(!is_rainbow("6h5h4h"));
    }

    #[test]
    #[should_panic]
    fn test_is_rainbow_invalid_flop() {
        is_rainbow("1c6h5s");
    }

    #[test]
    fn test_is_twotone() {
        assert!(is_twotone("9s5s4c"));
        assert!(is_twotone("9s5d4d"));
        assert!(!is_twotone("9c5h4s"));
        assert!(!is_twotone("9c5c4c"));
    }

    #[test]
    #[should_panic]
    fn test_is_twotone_invalid_flop() {
        is_twotone("216h5s");
    }

    #[test]
    fn test_is_monotone() {
        assert!(is_monotone("Th5h4h"));
        assert!(is_monotone("Tc5c4c"));
        assert!(!is_monotone("Tc5h4s"));
        assert!(!is_monotone("Tc5s4s"));
    }

    #[test]
    #[should_panic]
    fn test_is_monotone_invalid_flop() {
        is_monotone("7h6h6h");
    }

    #[test]
    fn test_get_suit_type() {
        assert_eq!(get_suit_type("JcTh7s"), FlopSuitType::Rainbow);
        assert_eq!(get_suit_type("JcTh7h"), FlopSuitType::Twotone);
        assert_eq!(get_suit_type("JsTs7s"), FlopSuitType::Montone);
    }

    #[test]
    #[should_panic]
    fn test_get_suit_type_invalid_flop() {
        get_suit_type("As Kh Td");
    }

    #[test]
    fn test_flop_suit_type_from_str() {
        assert_eq!(FlopSuitType::from_str("R"), FlopSuitType::Rainbow);
        assert_eq!(FlopSuitType::from_str("T"), FlopSuitType::Twotone);
        assert_eq!(FlopSuitType::from_str("M"), FlopSuitType::Montone);
    }

    #[test]
    #[should_panic]
    fn test_flop_suit_type_from_str_invalid_str() {
        FlopSuitType::from_str("invalid");
    }

    #[test]
    fn test_flop_suit_type_as_str() {
        assert_eq!(FlopSuitType::Rainbow.as_str(), "R");
        assert_eq!(FlopSuitType::Twotone.as_str(), "T");
        assert_eq!(FlopSuitType::Montone.as_str(), "M");
    }
}
