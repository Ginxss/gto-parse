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

pub fn is_valid_value(value: &i8) -> bool {
    match value {
        0..=12 => true,
        _ => false,
    }
}
