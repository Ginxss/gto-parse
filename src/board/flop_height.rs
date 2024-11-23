use crate::board::util::{
    properties::{num_category, RankCategory},
    validators::is_valid_flop,
};

pub fn is_1bw(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    num_category(flop, &RankCategory::Broadway) == 1
}

pub fn is_2bw(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    num_category(flop, &RankCategory::Broadway) == 2
}

pub fn is_3bw(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    num_category(flop, &RankCategory::Broadway) == 3
}

pub fn is_middling(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    num_category(flop, &RankCategory::Broadway) == 0
        && num_category(flop, &RankCategory::Middling) > 0
}

pub fn is_low(flop: &str) -> bool {
    assert!(is_valid_flop(flop));

    num_category(flop, &RankCategory::Low) == 3
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
}
