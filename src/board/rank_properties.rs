#[derive(Debug, PartialEq)]
pub enum RankCategory {
    Broadway,
    Middling,
    Low,
}

pub fn get_category(rank: &char) -> RankCategory {
    match rank {
        '2' | '3' | '4' | '5' | '6' => RankCategory::Low,
        '9' | '8' | '7' => RankCategory::Middling,
        'T' | 'J' | 'Q' | 'K' | 'A' => RankCategory::Broadway,
        _ => panic!("Invalid rank"),
    }
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
        _ => panic!("Invalid rank"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_category() {
        assert_eq!(get_category(&'A'), RankCategory::Broadway);
        assert_eq!(get_category(&'K'), RankCategory::Broadway);
        assert_eq!(get_category(&'Q'), RankCategory::Broadway);
        assert_eq!(get_category(&'J'), RankCategory::Broadway);
        assert_eq!(get_category(&'T'), RankCategory::Broadway);
        assert_eq!(get_category(&'9'), RankCategory::Middling);
        assert_eq!(get_category(&'8'), RankCategory::Middling);
        assert_eq!(get_category(&'7'), RankCategory::Middling);
        assert_eq!(get_category(&'6'), RankCategory::Low);
        assert_eq!(get_category(&'5'), RankCategory::Low);
        assert_eq!(get_category(&'4'), RankCategory::Low);
        assert_eq!(get_category(&'3'), RankCategory::Low);
        assert_eq!(get_category(&'2'), RankCategory::Low);
    }

    #[test]
    #[should_panic]
    fn test_get_category_invalid() {
        get_category(&'a');
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
