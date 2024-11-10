use crate::board::{
    flop_height::num_card_category,
    util::{
        accessors::get_ranks,
        rank_properties::{get_value, RankCategory},
        validators::{is_valid_flop, is_valid_rank, is_wheel_rank},
    },
};

fn get_diffs(ranks: &Vec<char>) -> Vec<i8> {
    assert!(ranks.len() == 3);
    assert!(ranks.iter().all(|rank| is_valid_rank(rank)));

    let diffs: Vec<i8> = ranks
        .windows(2)
        .map(|ranks| get_value(&ranks[0]) - get_value(&ranks[1]))
        .map(|diff| diff.abs())
        .collect();

    assert!(diffs.len() == 2);
    assert!(diffs.iter().all(|diff| *diff >= 0 && *diff <= 12));

    diffs
}

pub fn is_straight_possible(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    let ranks = get_ranks(flop);
    let diffs = get_diffs(&ranks);

    ranks_allow_straight(&ranks, &diffs)
}

// Only OESD, no straight
pub fn is_oesd_possible(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    let ranks = get_ranks(flop);
    let diffs = get_diffs(&ranks);

    let no_straight = !ranks_allow_straight(&ranks, &diffs);
    let oesd = diffs.iter().any(|diff| *diff <= 3);
    let ahi_gutshot = ranks.contains(&'A') && num_card_category(flop, RankCategory::Broadway) == 2;

    no_straight && oesd && !ahi_gutshot
}

// Only gutshot, no OESD or straight
pub fn is_gutshot_possible(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    let ranks = get_ranks(flop);
    let diffs = get_diffs(&ranks);

    let no_straight = !ranks_allow_straight(&ranks, &diffs);
    let normal_gutshot = diffs.iter().any(|diff| *diff == 4);
    // TODO: Besser mit alternate mapping mit A = -1?
    let ahi_gutshot = ranks.contains(&'A')
        && (num_card_category(flop, RankCategory::Broadway) == 2
            || ranks.iter().filter(|rank| is_wheel_rank(rank)).count() == 2);

    no_straight && (normal_gutshot || ahi_gutshot)
}

fn ranks_allow_straight(ranks: &Vec<char>, diffs: &Vec<i8>) -> bool {
    let normal_straight_possible = diffs.iter().sum::<i8>() <= 4;
    let wheel_straight_possible = ranks.iter().all(|rank| is_wheel_rank(rank));

    normal_straight_possible || wheel_straight_possible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_straight_possible() {
        assert!(is_straight_possible("Ts9c8h"));
        assert!(is_straight_possible("Ts9c7h"));
        assert!(is_straight_possible("Ts9c6h"));
        assert!(!is_straight_possible("Ts9c5h"));
        assert!(is_straight_possible("As2c3h"));
        assert!(!is_straight_possible("Ks2c3h"));
    }

    #[test]
    fn test_is_oesd_possible() {
        assert!(is_oesd_possible("Ts9c5h"));
        assert!(is_oesd_possible("Jh9h2h"));
        assert!(is_oesd_possible("Qh9h2h"));
        assert!(is_oesd_possible("Jd6c3c"));
        assert!(is_oesd_possible("Ks2c3h"));
        assert!(!is_oesd_possible("As2c3h"));
        assert!(!is_oesd_possible("AsJc8h"));
        assert!(!is_oesd_possible("AsKc8h"));
        assert!(!is_oesd_possible("Ks8h3c"));
    }

    #[test]
    fn test_is_gutshot_possible() {
        assert!(is_gutshot_possible("Ks8c4h"));
        assert!(!is_gutshot_possible("Ks8c5h"));
        assert!(is_gutshot_possible("Ks9c4h"));
        assert!(!is_gutshot_possible("KsJc4h"));
        assert!(!is_gutshot_possible("Ac2h4h"));
        assert!(!is_gutshot_possible("Ac2h5h"));
        assert!(is_gutshot_possible("Ac2h6h"));
        assert!(is_gutshot_possible("Ac9h5h"));
        assert!(is_gutshot_possible("AcKh6h"));
        assert!(is_gutshot_possible("Ac9h4h"));
    }
}
