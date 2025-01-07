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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_unique() {
        assert_eq!(sorted_unique(&vec!['K', '5', 'K']), vec!['K', '5']);
        assert_eq!(sorted_unique(&vec!['K', '5', '7']), vec!['K', '7', '5']);
        assert_eq!(sorted_unique(&vec!['2', '5', '6']), vec!['6', '5', '2']);
        assert_eq!(sorted_unique(&vec!['J', '8', '6']), vec!['J', '8', '6']);
        assert_eq!(sorted_unique(&vec!['8', '8', '8']), vec!['8']);
    }

    #[test]
    #[should_panic]
    fn test_sorted_unique_vec_too_short() {
        sorted_unique(&vec![]);
    }

    #[test]
    #[should_panic]
    fn test_sorted_unique_vec_too_long() {
        sorted_unique(&vec!['A', 'K', 'T', 'J']);
    }

    #[test]
    #[should_panic]
    fn test_sorted_unique_invalid_rank() {
        sorted_unique(&vec!['A', 'K', '1']);
    }

    #[test]
    fn test_is_straight_possible() {
        assert!(is_straight_possible("Ts9c8h"));
        assert!(is_straight_possible("7s9cTh"));
        assert!(is_straight_possible("Ts9c6h"));
        assert!(is_straight_possible("As2c3h"));
        assert!(is_straight_possible("AcKsTh"));
        assert!(!is_straight_possible("7s9c7h"));
        assert!(!is_straight_possible("Ts9c5h"));
        assert!(!is_straight_possible("Ks2c3h"));
        assert!(!is_straight_possible("AcKs3h"));
        assert!(!is_straight_possible("KhTsTc"));
    }

    #[test]
    #[should_panic]
    fn test_is_straight_possible_invalid_flop() {
        is_straight_possible("ÄsKsTd");
    }

    #[test]
    fn test_is_oesd_possible() {
        assert!(is_oesd_possible("Ts9c5h"));
        assert!(is_oesd_possible("Jh9h2h"));
        assert!(is_oesd_possible("Qh9h2h"));
        assert!(is_oesd_possible("Jd6c3c"));
        assert!(is_oesd_possible("Ks2c3h"));
        assert!(is_oesd_possible("As5c6h"));
        assert!(is_oesd_possible("As7c4h"));
        assert!(is_oesd_possible("AsTc7h"));
        assert!(is_oesd_possible("KsThTc"));
        assert!(!is_oesd_possible("AsTc6h"));
        assert!(!is_oesd_possible("As2c3h"));
        assert!(!is_oesd_possible("As8c8h"));
        assert!(!is_oesd_possible("AsJc7h"));
        assert!(!is_oesd_possible("AsKc8h"));
        assert!(!is_oesd_possible("Ks8h3c"));
        assert!(!is_oesd_possible("Ks8h8c"));
    }

    #[test]
    #[should_panic]
    fn test_is_oesd_possible_invalid_flop() {
        is_oesd_possible("AsKsÜd");
    }

    #[test]
    fn test_is_gutshot_possible() {
        assert!(is_gutshot_possible("Ks8c4h"));
        assert!(is_gutshot_possible("Ks9c4h"));
        assert!(is_gutshot_possible("Ac2h6h"));
        assert!(is_gutshot_possible("Ac9h5h"));
        assert!(is_gutshot_possible("AcKh6h"));
        assert!(is_gutshot_possible("Ac9h4h"));
        assert!(!is_gutshot_possible("AcKhTh"));
        assert!(!is_gutshot_possible("Ks8c5h"));
        assert!(!is_gutshot_possible("KsJc4h"));
        assert!(!is_gutshot_possible("Ac2h4h"));
        assert!(!is_gutshot_possible("Ac5h6s"));
        assert!(!is_gutshot_possible("7c5c5s"));
    }

    #[test]
    #[should_panic]
    fn test_is_gutshot_possible_invalid_flop() {
        is_gutshot_possible(" AsKsTd");
    }

    #[test]
    fn test_get_connectedness() {
        assert_eq!(get_connectedness("Ks8h3c"), FlopConnectedness::Disconnected);
        assert_eq!(get_connectedness("Ks9h3c"), FlopConnectedness::Gutshot);
        assert_eq!(get_connectedness("KsTh3c"), FlopConnectedness::OESD);
        assert_eq!(get_connectedness("KsTh9c"), FlopConnectedness::Straight);
    }

    #[test]
    #[should_panic]
    fn test_get_connectedness_invalid_flop() {
        get_connectedness("AsKsTd ");
    }

    #[test]
    fn test_flop_connectedness_from_str() {
        assert_eq!(
            FlopConnectedness::from_str("DC"),
            FlopConnectedness::Disconnected
        );
        assert_eq!(
            FlopConnectedness::from_str("GS"),
            FlopConnectedness::Gutshot
        );
        assert_eq!(FlopConnectedness::from_str("OESD"), FlopConnectedness::OESD);
        assert_eq!(
            FlopConnectedness::from_str("STR"),
            FlopConnectedness::Straight
        );
    }

    #[test]
    #[should_panic]
    fn test_flop_connectedness_from_str_invalid_str() {
        FlopConnectedness::from_str("invalid");
    }

    #[test]
    fn test_flop_connectedness_as_str() {
        assert_eq!(FlopConnectedness::Disconnected.as_str(), "DC");
        assert_eq!(FlopConnectedness::Gutshot.as_str(), "GS");
        assert_eq!(FlopConnectedness::OESD.as_str(), "OESD");
        assert_eq!(FlopConnectedness::Straight.as_str(), "STR");
    }
}
