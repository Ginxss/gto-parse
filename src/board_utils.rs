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
