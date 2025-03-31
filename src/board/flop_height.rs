use crate::board::util::{
    properties::{num_rank_category, RankCategory},
    validators::is_valid_flop,
};

#[derive(Debug, PartialEq)]
pub enum FlopHeight {
    SingleBW,
    DoubleBW,
    TripleBW,
    Middling,
    Low,
}

impl FlopHeight {
    pub fn from_str(string: &str) -> FlopHeight {
        match string {
            "1BW" => FlopHeight::SingleBW,
            "2BW" => FlopHeight::DoubleBW,
            "3BW" => FlopHeight::TripleBW,
            "MID" => FlopHeight::Middling,
            "LOW" => FlopHeight::Low,
            other => panic!("Invalid flop height string: {}", other),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            FlopHeight::SingleBW => "1BW",
            FlopHeight::DoubleBW => "2BW",
            FlopHeight::TripleBW => "3BW",
            FlopHeight::Middling => "MID",
            FlopHeight::Low => "LOW",
        }
    }
}

pub fn is_1bw(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    num_rank_category(flop, &RankCategory::Broadway) == 1
}

pub fn is_2bw(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    num_rank_category(flop, &RankCategory::Broadway) == 2
}

pub fn is_3bw(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    num_rank_category(flop, &RankCategory::Broadway) == 3
}

pub fn is_middling(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    num_rank_category(flop, &RankCategory::Broadway) == 0
        && num_rank_category(flop, &RankCategory::Middling) > 0
}

pub fn is_low(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    num_rank_category(flop, &RankCategory::Low) == 3
}

pub fn is_height(flop: &str, height: &FlopHeight) -> bool {
    match height {
        FlopHeight::TripleBW => is_3bw(flop),
        FlopHeight::DoubleBW => is_2bw(flop),
        FlopHeight::SingleBW => is_1bw(flop),
        FlopHeight::Middling => is_middling(flop),
        FlopHeight::Low => is_low(flop),
    }
}

pub fn get_height(flop: &str) -> FlopHeight {
    if is_1bw(flop) {
        FlopHeight::SingleBW
    } else if is_2bw(flop) {
        FlopHeight::DoubleBW
    } else if is_3bw(flop) {
        FlopHeight::TripleBW
    } else if is_middling(flop) {
        FlopHeight::Middling
    } else if is_low(flop) {
        FlopHeight::Low
    } else {
        panic!("Invalid or unsupportted flop height: {}", flop);
    }
}
