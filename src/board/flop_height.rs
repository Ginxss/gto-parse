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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_1bw() {
        assert!(is_1bw("Ac9h6s"));
        assert!(is_1bw("Tc9h6s"));
        assert!(!is_1bw("9c8h6s"));
        assert!(!is_1bw("Kc4hQs"));
        assert!(!is_1bw("3c2h6s"));
    }

    #[test]
    #[should_panic]
    fn test_is_1bw_invalid_flop() {
        is_1bw("123a");
    }

    #[test]
    fn test_is_2bw() {
        assert!(is_2bw("Kc4hQs"));
        assert!(is_2bw("AcTh6s"));
        assert!(!is_2bw("Tc9h6s"));
        assert!(!is_2bw("9c2h6s"));
        assert!(!is_2bw("3c2h6s"));
    }

    #[test]
    #[should_panic]
    fn test_is_2bw_invalid_flop() {
        is_2bw("123ab");
    }

    #[test]
    fn test_is_3bw() {
        assert!(is_3bw("KcJhQs"));
        assert!(is_3bw("AcKhTs"));
        assert!(!is_3bw("AcKh6s"));
        assert!(!is_3bw("Tc8s2s"));
        assert!(!is_3bw("3c2h5s"));
    }

    #[test]
    #[should_panic]
    fn test_is_3bw_invalid_flop() {
        is_3bw("123abc");
    }

    #[test]
    fn test_is_middling() {
        assert!(is_middling("9h3c2c"));
        assert!(is_middling("8h7c2c"));
        assert!(!is_middling("9h3cAc"));
        assert!(!is_middling("6h3c2c"));
        assert!(!is_middling("AcKh6s"));
        assert!(!is_middling("AcKhTs"));
    }

    #[test]
    #[should_panic]
    fn test_is_middling_invalid_flop() {
        is_middling("abc123");
    }

    #[test]
    fn test_is_low() {
        assert!(is_low("6h5h4h"));
        assert!(is_low("5c2h3s"));
        assert!(is_low("5c2h2s"));
        assert!(!is_low("Ac2h3s"));
        assert!(!is_low("Tc8sKs"));
        assert!(!is_low("7c8s6s"));
        assert!(!is_low("Jc8sJs"));
    }

    #[test]
    #[should_panic]
    fn test_is_low_invalid_flop() {
        is_low("5c2h2h");
    }

    #[test]
    fn test_get_height() {
        assert_eq!(get_height("Qd8h2s"), FlopHeight::SingleBW);
        assert_eq!(get_height("QdJh2s"), FlopHeight::DoubleBW);
        assert_eq!(get_height("QdJhTs"), FlopHeight::TripleBW);
        assert_eq!(get_height("9d8h2s"), FlopHeight::Middling);
        assert_eq!(get_height("3d4h4s"), FlopHeight::Low);
    }

    #[test]
    #[should_panic]
    fn test_get_height_invalid_flop() {
        get_height("5h5h5h");
    }

    #[test]
    fn test_flop_height_from_str() {
        assert_eq!(FlopHeight::from_str("1BW"), FlopHeight::SingleBW);
        assert_eq!(FlopHeight::from_str("2BW"), FlopHeight::DoubleBW);
        assert_eq!(FlopHeight::from_str("3BW"), FlopHeight::TripleBW);
        assert_eq!(FlopHeight::from_str("MID"), FlopHeight::Middling);
        assert_eq!(FlopHeight::from_str("LOW"), FlopHeight::Low);
    }

    #[test]
    #[should_panic]
    fn test_flop_height_from_str_invalid_str() {
        FlopHeight::from_str("invalid");
    }

    #[test]
    fn test_flop_height_as_str() {
        assert_eq!(FlopHeight::SingleBW.as_str(), "1BW");
        assert_eq!(FlopHeight::DoubleBW.as_str(), "2BW");
        assert_eq!(FlopHeight::TripleBW.as_str(), "3BW");
        assert_eq!(FlopHeight::Middling.as_str(), "MID");
        assert_eq!(FlopHeight::Low.as_str(), "LOW");
    }
}
