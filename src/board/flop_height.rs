use super::util::{
    accessors::get_ranks,
    rank_properties::{get_category, RankCategory},
};

pub fn num_card_category(flop: &str, filter_category: RankCategory) -> usize {
    get_ranks(flop)
        .iter()
        .map(|rank| get_category(&rank))
        .filter(|category| *category == filter_category)
        .count()
}

pub fn is_1bw(flop: &str) -> bool {
    num_card_category(flop, RankCategory::Broadway) == 1
}

pub fn is_2bw(flop: &str) -> bool {
    num_card_category(flop, RankCategory::Broadway) == 2
}

pub fn is_3bw(flop: &str) -> bool {
    num_card_category(flop, RankCategory::Broadway) == 3
}

pub fn is_middling(flop: &str) -> bool {
    num_card_category(flop, RankCategory::Broadway) == 0
        && num_card_category(flop, RankCategory::Middling) > 0
}

pub fn is_low(flop: &str) -> bool {
    num_card_category(flop, RankCategory::Broadway) == 0
        && num_card_category(flop, RankCategory::Middling) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_card_category() {
        let flop = "3h9cTh";
        assert_eq!(num_card_category(flop, RankCategory::Broadway), 1);
        assert_eq!(num_card_category(flop, RankCategory::Middling), 1);
        assert_eq!(num_card_category(flop, RankCategory::Low), 1);

        let flop = "AsAc6h";
        assert_eq!(num_card_category(flop, RankCategory::Broadway), 2);
        assert_eq!(num_card_category(flop, RankCategory::Middling), 0);
        assert_eq!(num_card_category(flop, RankCategory::Low), 1);

        let flop = "Qc9h7h";
        assert_eq!(num_card_category(flop, RankCategory::Broadway), 1);
        assert_eq!(num_card_category(flop, RankCategory::Middling), 2);
        assert_eq!(num_card_category(flop, RankCategory::Low), 0);
    }

    #[test]
    fn test_is_1bw() {
        assert!(is_1bw("Ac9h6s"));
        assert!(is_1bw("Tc9h6s"));
        assert!(!is_1bw("9c8h6s"));
        assert!(!is_1bw("Kc4hQs"));
    }

    #[test]
    fn test_is_2bw() {
        assert!(is_2bw("Kc4hQs"));
        assert!(is_2bw("AcTh6s"));
        assert!(!is_2bw("Tc9h6s"));
        assert!(!is_2bw("9c2h6s"));
    }

    #[test]
    fn test_is_3bw() {
        assert!(is_3bw("KcJhQs"));
        assert!(is_3bw("AcKhTs"));
        assert!(!is_3bw("AcKh6s"));
        assert!(!is_3bw("Tc8s2s"));
    }

    #[test]
    fn test_is_middling() {
        assert!(is_middling("9h3c2c"));
        assert!(is_middling("8h7c2c"));
        assert!(!is_middling("9h3cAc"));
        assert!(!is_middling("6h3c2c"));
    }

    #[test]
    fn test_is_low() {
        assert!(is_low("6h5h4h"));
        assert!(is_low("5c3h2s"));
        assert!(!is_low("Ac2h3s"));
        assert!(!is_low("Tc8sKs"));
    }
}
