use core::panic;
use std::cmp::Ordering;

const BW_CARDS: [char; 5] = ['A', 'K', 'Q', 'J', 'T'];
const MID_CARDS: [char; 3] = ['9', '8', '7'];
const LOW_CARDS: [char; 5] = ['6', '5', '4', '3', '2'];

fn get_first_card(board: &str) -> char {
    board.chars().next().expect("Board not long enough")
}

fn get_second_card(board: &str) -> char {
    board.chars().nth(2).expect("Board not long enough")
}

fn get_third_card(board: &str) -> char {
    board.chars().nth(4).expect("Board not long enough")
}

fn get_first_suit(board: &str) -> char {
    board.chars().nth(1).expect("Board not long enough")
}

fn get_second_suit(board: &str) -> char {
    board.chars().nth(3).expect("Board not long enough")
}

fn get_third_suit(board: &str) -> char {
    board.chars().nth(5).expect("Board not long enough")
}

pub fn is_1bw(board: &str) -> bool {
    BW_CARDS.contains(&get_first_card(board))
        && !BW_CARDS.contains(&get_second_card(board))
        && !BW_CARDS.contains(&get_third_card(board))
}

pub fn is_2bw(board: &str) -> bool {
    BW_CARDS.contains(&get_first_card(board))
        && BW_CARDS.contains(&get_second_card(board))
        && !BW_CARDS.contains(&get_third_card(board))
}

pub fn is_3bw(board: &str) -> bool {
    BW_CARDS.contains(&get_first_card(board))
        && BW_CARDS.contains(&get_second_card(board))
        && BW_CARDS.contains(&get_third_card(board))
}

pub fn is_middling(board: &str) -> bool {
    MID_CARDS.contains(&get_first_card(board))
}

pub fn is_low(board: &str) -> bool {
    LOW_CARDS.contains(&get_first_card(board))
}

pub fn is_rainbow(board: &str) -> bool {
    let first_suit = get_first_suit(board);
    let second_suit = get_second_suit(board);
    let third_suit = get_third_suit(board);

    first_suit != second_suit && first_suit != third_suit && second_suit != third_suit
}

pub fn is_twotone(board: &str) -> bool {
    let first_suit = get_first_suit(board);
    let second_suit = get_second_suit(board);
    let third_suit = get_third_suit(board);

    (first_suit == second_suit && first_suit != third_suit)
        || (first_suit == third_suit && first_suit != second_suit)
        || (second_suit == third_suit && second_suit != first_suit)
}

pub fn is_monotone(board: &str) -> bool {
    let first_suit = get_first_suit(board);
    first_suit == get_second_suit(board) && first_suit == get_third_suit(board)
}

fn get_rank(card: &char) -> i8 {
    match card {
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
        _ => panic!("Invalid card"),
    }
}

fn sort(board: &str) -> String {
    assert!(board.len() == 6);

    let mut individual_cards = [&board[..2], &board[2..4], &board[4..]];

    individual_cards.sort_by(|card1, card2| {
        let card1_rank = get_rank(&card1.chars().next().expect("Card should be there"));
        let card2_rank = get_rank(&card2.chars().next().expect("Card should be there"));
        let diff = card2_rank - card1_rank;
        if diff > 0 {
            Ordering::Greater
        } else if diff < 0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    individual_cards.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let board = "KsAcTh";
        let sorted_board = sort(board);

        assert!(sorted_board == "AcKsTh");
    }
}
