use std::cmp::Ordering;

use crate::board::util::{
    accessors::get_ranks,
    properties::{get_value, num_rank_category, RankCategory},
    validators::{is_valid_flop, is_valid_rank, is_valid_value, is_wheel_rank},
};

#[derive(Debug, PartialEq)]
pub enum FlopConnectedness {
    Disconnected,
    Gutshot,
    OESD,
    Straight,
}

impl FlopConnectedness {
    pub fn from_str(string: &str) -> FlopConnectedness {
        match string {
            "DC" => FlopConnectedness::Disconnected,
            "GS" => FlopConnectedness::Gutshot,
            "OESD" => FlopConnectedness::OESD,
            "STR" => FlopConnectedness::Straight,
            other => panic!("Invalid flop connectedness string: {}", other),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            FlopConnectedness::Disconnected => "DC",
            FlopConnectedness::Gutshot => "GS",
            FlopConnectedness::OESD => "OESD",
            FlopConnectedness::Straight => "STR",
        }
    }
}

fn sorted_unique(ranks: &Vec<char>) -> Vec<char> {
    assert!((1..=3).contains(&ranks.len()));
    assert!(ranks.iter().all(|rank| is_valid_rank(rank)));

    let mut ranks = ranks.clone();
    let original_len = ranks.len();

    ranks.sort_by(|rank1, rank2| {
        let diff = get_value(rank1) - get_value(rank2);
        if diff < 0 {
            Ordering::Less
        } else if diff > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
        .reverse()
    });
    ranks.dedup();

    assert!(ranks.len() <= original_len);
    assert!(ranks.iter().all(|rank| is_valid_rank(rank)));

    ranks
}

fn get_diffs(ranks: &Vec<char>) -> Vec<i8> {
    assert!((1..=3).contains(&ranks.len()));
    assert!(ranks.iter().all(|rank| is_valid_rank(rank)));

    let ranks = sorted_unique(ranks);
    let diffs: Vec<i8> = ranks
        .windows(2)
        .map(|ranks| get_value(&ranks[0]) - get_value(&ranks[1]))
        .map(|diff| diff.abs())
        .collect();

    assert!(diffs.len() == ranks.len() - 1);
    assert!(diffs.iter().all(|diff| is_valid_value(diff)));

    diffs
}

pub fn is_straight_possible(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    let ranks = sorted_unique(&get_ranks(flop));
    if ranks.len() < 3 {
        return false;
    }

    let normal_straight = get_diffs(&ranks).iter().sum::<i8>() <= 4;
    let wheel_straight = ranks.iter().all(|rank| is_wheel_rank(rank));

    normal_straight || wheel_straight
}

// Only OESD, no straight
pub fn is_oesd_possible(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    if is_straight_possible(flop) {
        return false;
    }

    let ranks_no_a = get_ranks(flop)
        .into_iter()
        .filter(|rank| *rank != 'A')
        .collect();

    get_diffs(&ranks_no_a).iter().any(|diff| *diff <= 3)
}

// Only gutshot, no OESD or straight
pub fn is_gutshot_possible(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    if is_straight_possible(flop) || is_oesd_possible(flop) {
        return false;
    }

    let ranks = sorted_unique(&get_ranks(flop));

    let normal_gutshot = get_diffs(&ranks).iter().any(|diff| *diff == 4);

    let ahi = ranks.contains(&'A');
    let another_bw = num_rank_category(flop, &RankCategory::Broadway) == 2;
    let another_wheel = ranks.iter().filter(|rank| is_wheel_rank(rank)).count() == 2;
    let ahi_gutshot = ahi && (another_bw || another_wheel);

    normal_gutshot || ahi_gutshot
}

pub fn is_disconnected(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    !is_gutshot_possible(flop) && !is_oesd_possible(flop) && !is_straight_possible(flop)
}

pub fn get_connectedness(flop: &str) -> FlopConnectedness {
    if is_straight_possible(flop) {
        FlopConnectedness::Straight
    } else if is_oesd_possible(flop) {
        FlopConnectedness::OESD
    } else if is_gutshot_possible(flop) {
        FlopConnectedness::Gutshot
    } else {
        FlopConnectedness::Disconnected
    }
}

pub fn is_connectedness(flop: &str, connectedness: &FlopConnectedness) -> bool {
    match connectedness {
        FlopConnectedness::Straight => is_straight_possible(flop),
        FlopConnectedness::OESD => is_oesd_possible(flop),
        FlopConnectedness::Gutshot => is_gutshot_possible(flop),
        FlopConnectedness::Disconnected => is_disconnected(flop),
    }
}
