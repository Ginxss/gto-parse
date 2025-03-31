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

pub fn is_suit_type(flop: &str, suit: &FlopSuitType) -> bool {
    match suit {
        FlopSuitType::Rainbow => is_rainbow(flop),
        FlopSuitType::Twotone => is_twotone(flop),
        FlopSuitType::Montone => is_monotone(flop),
    }
}
