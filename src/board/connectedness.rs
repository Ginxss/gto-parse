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

    diffs
}

pub fn is_straight_possible(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    ranks_allow_straight(&get_ranks(flop))
}

// Only OESD, no straight
pub fn is_oesd_possible(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    let ranks = get_ranks(flop);
    let diffs = get_diffs(&ranks);

    let no_straight_possible = !ranks_allow_straight(&ranks);
    let oesd_possible = no_straight_possible && diffs.iter().any(|diff| *diff <= 3);

    let a_draw = ranks.iter().any(|rank| *rank == 'A')
        && num_card_category(flop, RankCategory::Broadway) == 2;

    oesd_possible && !a_draw
}

fn ranks_allow_straight(ranks: &Vec<char>) -> bool {
    let normal_straight_possible = get_diffs(&ranks).iter().sum::<i8>() <= 4;
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
}
