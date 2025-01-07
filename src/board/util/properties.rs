use crate::board::util::{accessors::get_ranks, validators::is_valid_flop};

#[derive(Debug, PartialEq)]
pub enum RankCategory {
    Broadway,
    Middling,
    Low,
}

pub fn get_rank_category(rank: &char) -> RankCategory {
    match rank {
        '2' | '3' | '4' | '5' | '6' => RankCategory::Low,
        '9' | '8' | '7' => RankCategory::Middling,
        'T' | 'J' | 'Q' | 'K' | 'A' => RankCategory::Broadway,
        invalid_rank => panic!("Invalid rank: {}", invalid_rank),
    }
}

pub fn num_rank_category(flop: &str, filter_category: &RankCategory) -> usize {
    assert!(is_valid_flop(flop));

    let num_category = get_ranks(flop)
        .iter()
        .map(|rank| get_rank_category(&rank))
        .filter(|category| category == filter_category)
        .count();

    assert!(num_category <= 3);

    num_category
}

pub fn get_value(rank: &char) -> i8 {
    match rank {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        invalid_rank => panic!("Invalid rank: {}", invalid_rank),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_category() {
        assert_eq!(get_rank_category(&'A'), RankCategory::Broadway);
        assert_eq!(get_rank_category(&'K'), RankCategory::Broadway);
        assert_eq!(get_rank_category(&'Q'), RankCategory::Broadway);
        assert_eq!(get_rank_category(&'J'), RankCategory::Broadway);
        assert_eq!(get_rank_category(&'T'), RankCategory::Broadway);
        assert_eq!(get_rank_category(&'9'), RankCategory::Middling);
        assert_eq!(get_rank_category(&'8'), RankCategory::Middling);
        assert_eq!(get_rank_category(&'7'), RankCategory::Middling);
        assert_eq!(get_rank_category(&'6'), RankCategory::Low);
        assert_eq!(get_rank_category(&'5'), RankCategory::Low);
        assert_eq!(get_rank_category(&'4'), RankCategory::Low);
        assert_eq!(get_rank_category(&'3'), RankCategory::Low);
        assert_eq!(get_rank_category(&'2'), RankCategory::Low);
    }

    #[test]
    #[should_panic]
    fn test_get_category_invalid() {
        get_rank_category(&'a');
    }

    #[test]
    fn test_num_category() {
        let flop = "3h9cTh";
        assert_eq!(num_rank_category(flop, &RankCategory::Broadway), 1);
        assert_eq!(num_rank_category(flop, &RankCategory::Middling), 1);
        assert_eq!(num_rank_category(flop, &RankCategory::Low), 1);

        let flop = "AsAc6h";
        assert_eq!(num_rank_category(flop, &RankCategory::Broadway), 2);
        assert_eq!(num_rank_category(flop, &RankCategory::Middling), 0);
        assert_eq!(num_rank_category(flop, &RankCategory::Low), 1);

        let flop = "Qc9h7h";
        assert_eq!(num_rank_category(flop, &RankCategory::Broadway), 1);
        assert_eq!(num_rank_category(flop, &RankCategory::Middling), 2);
        assert_eq!(num_rank_category(flop, &RankCategory::Low), 0);
    }

    #[test]
    #[should_panic]
    fn test_num_category_invalid() {
        num_rank_category("3d0cAh", &RankCategory::Broadway);
    }

    #[test]
    fn test_get_value() {
        assert_eq!(get_value(&'A'), 12);
        assert_eq!(get_value(&'K'), 11);
        assert_eq!(get_value(&'Q'), 10);
        assert_eq!(get_value(&'J'), 9);
        assert_eq!(get_value(&'T'), 8);
        assert_eq!(get_value(&'9'), 7);
        assert_eq!(get_value(&'8'), 6);
        assert_eq!(get_value(&'7'), 5);
        assert_eq!(get_value(&'6'), 4);
        assert_eq!(get_value(&'5'), 3);
        assert_eq!(get_value(&'4'), 2);
        assert_eq!(get_value(&'3'), 1);
        assert_eq!(get_value(&'2'), 0);
    }

    #[test]
    #[should_panic]
    fn test_get_value_invalid() {
        get_value(&'b');
    }
}
