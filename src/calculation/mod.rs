pub mod datarow;

use std::{
    collections::HashSet,
    fs::{self, DirEntry},
    path::Path,
};

use datarow::DataRow;

use crate::{
    args::Args,
    files,
    poker::{action::Action, betsize::Betsize, board::Board, position::Positions},
};

const DATA_DIR: &str = "./data";

pub fn build_data_rows_with_boards(args: Args) -> (Vec<DataRow>, Vec<Board>) {
    let size_dirs = get_size_dirs(&args.positions, DATA_DIR);

    let (datarows, considered_boards) = args
        .betsizes
        .iter()
        .map(|betsize| build_data_row_with_boards(betsize, &size_dirs, &args))
        .unzip();

    (datarows, validate_identical_and_get(considered_boards))
}

fn get_size_dirs(positions: &Positions, data_dir: &str) -> Vec<DirEntry> {
    let pos_dir = get_pos_dir(data_dir, positions);
    files::get_dirs(&pos_dir.path())
}

fn get_pos_dir(data_dir: &str, pos: &Positions) -> DirEntry {
    files::get_dirs(Path::new(data_dir))
        .into_iter()
        .find(|entry| {
            let name = files::get_name(&entry.path());
            return name.contains(&pos.ip.to_string()) && name.contains(&pos.oop.to_string());
        })
        .expect("Could not find position directory")
}

fn build_data_row_with_boards(
    betsize: &Betsize,
    size_dirs: &Vec<DirEntry>,
    args: &Args,
) -> (DataRow, Vec<Board>) {
    let size_dir = find_size_dir(betsize, size_dirs);
    let action_file = get_action_file_in_dir(size_dir, &args.actions);
    let file_content = fs::read_to_string(action_file.path()).expect("Could not read file content");

    let lines_with_boards = get_lines_with_boards(&file_content);
    let (filteres_lines, filtered_boards) = filter(lines_with_boards, args);

    let mut data_row = build_data_row(&filteres_lines);
    data_row.size = Some(betsize.clone());
    (data_row, filtered_boards)
}

fn find_size_dir<'a>(betsize: &Betsize, size_dirs: &'a Vec<DirEntry>) -> &'a DirEntry {
    size_dirs
        .iter()
        .find(|dir| files::get_name(&dir.path()) == betsize.to_string())
        .expect(&format!("Could not find size dir for {betsize}"))
}

fn get_action_file_in_dir(dir: &DirEntry, actions: &Vec<Action>) -> DirEntry {
    files::get_files(&dir.path())
        .into_iter()
        .find(|file| file_matches_actions(file, actions))
        .expect("Could not find action file")
}

fn file_matches_actions(file: &DirEntry, actions: &Vec<Action>) -> bool {
    let filename = files::get_name(&file.path());
    let split_pattern = '_';

    filename.matches(split_pattern).count() == actions.len()
        && filename
            .split(split_pattern)
            .skip(1)
            .zip(actions.iter())
            .all(|(action_name, action)| action_name == action.to_long_string())
}

fn get_lines_with_boards(file_content: &String) -> Vec<(&str, Board)> {
    let lines_with_boards = file_content
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| (line, extract_board(line)))
        .collect();

    validate_no_duplicates(&lines_with_boards);

    lines_with_boards
}

fn extract_board(line: &str) -> Board {
    let board_str = line
        .split('\t')
        .next()
        .expect("Error getting board from line");

    Board::try_from(board_str).unwrap()
}

fn validate_no_duplicates(lines_with_boards: &Vec<(&str, Board)>) {
    let boards: HashSet<&Board> = lines_with_boards.iter().map(|(_, board)| board).collect();
    assert_eq!(boards.len(), lines_with_boards.len());
}

fn filter<'a>(lines_with_boards: Vec<(&str, Board)>, args: &Args) -> (Vec<String>, Vec<Board>) {
    lines_with_boards
        .into_iter()
        .filter_map(|(line, board)| {
            board_matches_conditions(&board, args).then_some((line.to_string(), board))
        })
        .unzip()
}

fn board_matches_conditions(board: &Board, args: &Args) -> bool {
    let heights_match =
        args.heights.is_empty() || args.heights.iter().any(|height| board.is_height(&height));
    let suits_match = args.suits.is_empty() || args.suits.iter().any(|suit| board.is_suit(&suit));
    let connections_match = args.connections.is_empty()
        || args
            .connections
            .iter()
            .any(|connection| board.is_connection(&connection));
    let pair_match = args.pair.is_empty() || args.pair.iter().any(|pair| board.is_pair(&pair));

    heights_match && suits_match && connections_match && pair_match
}

fn build_data_row(lines: &Vec<String>) -> DataRow {
    let data_rows: Vec<DataRow> = lines.iter().map(|line| DataRow::new(line)).collect();
    let count = data_rows.len();
    assert!(count > 0, "No lines found matching conditions");

    data_rows
        .into_iter()
        .reduce(|row1, row2| row1 + row2)
        .map(|sum_row| sum_row / count)
        .expect("Could not calculate data row")
}

fn validate_identical_and_get(boards: Vec<Vec<Board>>) -> Vec<Board> {
    boards
        .into_iter()
        .reduce(|acc, board| {
            assert_eq!(acc, board);
            acc
        })
        .expect("Could not reduce considered boards")
}

#[cfg(test)]
mod tests {
    use files::get_name;

    use crate::poker::{
        board::{height::BoardHeight, pair::BoardPair, suit::BoardSuit},
        position::Position,
    };

    use super::*;

    const DATA_DIR: &str = "./test_data";

    #[test]
    fn test_get_size_dirs_names() {
        let positions = Positions {
            ip: Position::BTN,
            oop: Position::BB,
        };

        let mut size_dirs_names: Vec<_> = get_size_dirs(&positions, DATA_DIR)
            .into_iter()
            .map(|dir| get_name(&dir.path()))
            .collect();

        let mut expected_dir_names = vec![
            "33".to_string(),
            "50".to_string(),
            "75".to_string(),
            "150".to_string(),
        ];

        size_dirs_names.sort();
        expected_dir_names.sort();

        assert_eq!(size_dirs_names, expected_dir_names);
    }

    #[test]
    fn test_build_data_rows_with_boards_1() {
        let args = Args {
            positions: Positions {
                ip: Position::BTN,
                oop: Position::BB,
            },
            pair: Vec::new(),
            suits: Vec::new(),
            heights: Vec::new(),
            actions: vec![Action::Check],
            betsizes: vec![
                Betsize::Size33,
                Betsize::Size50,
                Betsize::Size75,
                Betsize::Size150,
            ],
            connections: Vec::new(),
        };

        let size_dirs = get_size_dirs(&args.positions, DATA_DIR);

        let (datarows, considered_boards): (Vec<DataRow>, Vec<Vec<Board>>) = args
            .betsizes
            .iter()
            .map(|betsize| build_data_row_with_boards(betsize, &size_dirs, &args))
            .unzip();

        let (datarows, considered_boards) =
            (datarows, validate_identical_and_get(considered_boards));

        let expected_datarows = vec![
            DataRow {
                size: Some(Betsize::Size33),
                eq: 60.755173,
                ev: 28.557,
                bet_freq: 28.126001,
                check_freq: 65.877335,
            },
            DataRow {
                size: Some(Betsize::Size50),
                eq: 52.90833,
                ev: 41.9065,
                bet_freq: 28.626001,
                check_freq: 75.374504,
            },
            DataRow {
                size: Some(Betsize::Size75),
                eq: 61.075,
                ev: 39.07317,
                bet_freq: 21.626001,
                check_freq: 69.04117,
            },
            DataRow {
                size: Some(Betsize::Size150),
                eq: 55.74167,
                ev: 31.739836,
                bet_freq: 27.459335,
                check_freq: 71.70783,
            },
        ];

        assert_eq!(datarows.len(), expected_datarows.len());
        assert!(expected_datarows.iter().all(|row| datarows.contains(row)));

        let expected_boards = vec![
            Board::try_from("8s8d8c").unwrap(),
            Board::try_from("6d8s8d").unwrap(),
            Board::try_from("4c7dKs").unwrap(),
            Board::try_from("5s5dAs").unwrap(),
            Board::try_from("4c7dAs").unwrap(),
            Board::try_from("4d6sTs").unwrap(),
        ];

        assert_eq!(considered_boards.len(), expected_boards.len());
        assert!(expected_boards
            .iter()
            .all(|board| considered_boards.contains(board)));
    }

    #[test]
    fn test_build_data_rows_with_boards_2() {
        let args = Args {
            positions: Positions {
                ip: Position::CO,
                oop: Position::BB,
            },
            pair: vec![BoardPair::Unpaired],
            suits: Vec::new(),
            heights: vec![BoardHeight::SingleBW, BoardHeight::Middling],
            actions: vec![Action::Check],
            betsizes: vec![Betsize::Size33, Betsize::Size75],
            connections: Vec::new(),
        };

        let size_dirs = get_size_dirs(&args.positions, DATA_DIR);

        let (datarows, considered_boards): (Vec<DataRow>, Vec<Vec<Board>>) = args
            .betsizes
            .iter()
            .map(|betsize| build_data_row_with_boards(betsize, &size_dirs, &args))
            .unzip();

        let (datarows, considered_boards) =
            (datarows, validate_identical_and_get(considered_boards));

        let expected_datarows = vec![
            DataRow {
                size: Some(Betsize::Size33),
                eq: 56.47,
                ev: 41.996334,
                bet_freq: 18.763334,
                check_freq: 58.896336,
            },
            DataRow {
                size: Some(Betsize::Size75),
                eq: 42.803333,
                ev: 47.996338,
                bet_freq: 20.43,
                check_freq: 58.563004,
            },
        ];

        assert_eq!(datarows.len(), expected_datarows.len());
        assert!(expected_datarows.iter().all(|row| datarows.contains(row)));

        let expected_boards = vec![
            Board::try_from("Ks7d4c").unwrap(),
            Board::try_from("As7d4c").unwrap(),
            Board::try_from("Ts6s4d").unwrap(),
        ];

        assert_eq!(considered_boards.len(), expected_boards.len());
        assert!(expected_boards
            .iter()
            .all(|board| considered_boards.contains(board)));
    }

    #[test]
    fn test_build_data_rows_with_boards_3() {
        let args = Args {
            positions: Positions {
                ip: Position::HJ,
                oop: Position::BB,
            },
            pair: vec![BoardPair::Paired, BoardPair::Trips],
            suits: vec![BoardSuit::Twotone],
            heights: Vec::new(),
            actions: vec![Action::Check],
            betsizes: vec![Betsize::Size50, Betsize::Size150],
            connections: Vec::new(),
        };

        let size_dirs = get_size_dirs(&args.positions, DATA_DIR);

        let (datarows, considered_boards): (Vec<DataRow>, Vec<Vec<Board>>) = args
            .betsizes
            .iter()
            .map(|betsize| build_data_row_with_boards(betsize, &size_dirs, &args))
            .unzip();

        let (datarows, considered_boards) =
            (datarows, validate_identical_and_get(considered_boards));

        let expected_datarows = vec![
            DataRow {
                size: Some(Betsize::Size50),
                eq: 72.239,
                ev: 27.4945,
                bet_freq: 23.442001,
                check_freq: 80.569,
            },
            DataRow {
                size: Some(Betsize::Size150),
                eq: 69.239,
                ev: 23.9945,
                bet_freq: 19.442001,
                check_freq: 83.569,
            },
        ];

        assert_eq!(datarows.len(), expected_datarows.len());
        assert!(expected_datarows.iter().all(|row| datarows.contains(row)));

        let expected_boards = vec![
            Board::try_from("8d8s6d").unwrap(),
            Board::try_from("As5d5s").unwrap(),
        ];

        assert_eq!(considered_boards.len(), expected_boards.len());
        assert!(expected_boards
            .iter()
            .all(|board| considered_boards.contains(board)));
    }
}
