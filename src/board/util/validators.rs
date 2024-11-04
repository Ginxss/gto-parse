use std::collections::HashSet;

pub fn is_valid_rank(rank: &char) -> bool {
    match rank {
        '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'T' | 'J' | 'Q' | 'K' | 'A' => true,
        _ => false,
    }
}

pub fn is_wheel_rank(rank: &char) -> bool {
    match rank {
        'A' | '2' | '3' | '4' | '5' => true,
        _ => false,
    }
}

pub fn is_valid_suit(suit: &char) -> bool {
    match suit {
        's' | 'd' | 'h' | 'c' => true,
        _ => false,
    }
}

pub fn is_valid_card(card: &str) -> bool {
    let mut chars = card.chars();

    card.len() == 2
        && is_valid_rank(&chars.next().unwrap())
        && is_valid_suit(&chars.next().unwrap())
}

pub fn is_valid_flop(flop: &str) -> bool {
    if flop.len() != 6 {
        return false;
    }

    let unique_cards = HashSet::from([&flop[..2], &flop[2..4], &flop[4..]]);

    unique_cards.len() == 3 && unique_cards.iter().all(|card| is_valid_card(card))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_rank() {
        assert!(
            ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
                .iter()
                .all(|rank| is_valid_rank(rank))
        );

        assert!(['a', 'ö', 't', 'R', 'L', '.', 's', 'h', 'd', 'c']
            .iter()
            .all(|rank| !is_valid_rank(rank)));
    }

    #[test]
    fn test_is_wheel_rank() {
        assert!(['A', '2', '3', '4', '5']
            .iter()
            .all(|rank| is_wheel_rank(rank)));

        assert!(
            ['6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'a', 'ö', 't', 'R', 'L', '.']
                .iter()
                .all(|rank| !is_wheel_rank(rank))
        );
    }

    #[test]
    fn test_is_valid_suit() {
        assert!(['s', 'd', 'h', 'c'].iter().all(|suit| is_valid_suit(suit)));

        assert!(['a', 'ö', 't', 'R', 'L', '.', 'A', 'T', '6', '2']
            .iter()
            .all(|rank| !is_valid_suit(rank)));
    }

    #[test]
    fn test_is_valid_card() {
        assert!(["As", "Kd", "6h", "2c"]
            .iter()
            .all(|card| is_valid_card(card)));

        assert!(["A", "js", "KK", "TD", "Tdd", "6d2c", "Qa", "QdQ"]
            .iter()
            .all(|card| !is_valid_card(card)));
    }

    #[test]
    fn test_is_valid_flop() {
        assert!(["AhKs7c", "Kd2d2c", "Ts6h8h", "KcKsKh"]
            .iter()
            .all(|flop| is_valid_flop(flop)));

        assert!([
            "AhKs", "AhKs9", "AhKs9l", "AhKs7hh", "AhKs7h8", "ahKs7c", "Kd2d2d", "n", "......"
        ]
        .iter()
        .all(|flop| !is_valid_flop(flop)));
    }
}
