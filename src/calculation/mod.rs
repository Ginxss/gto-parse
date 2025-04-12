pub mod datarow;

use std::{
    collections::HashSet,
    fs::{self, DirEntry},
    path::Path,
};

use datarow::DataRow;

use crate::{
    args::{action::Action, betsize::Betsize, position::Positions, Args},
    files,
    poker::board::Board,
};

const DATA_DIR: &str = "./data";

pub fn build_data_rows_with_boards(args: Args) -> (Vec<DataRow>, Vec<Board>) {
    let size_dirs = get_size_dirs(&args.positions);

    let (datarows, considered_boards) = args
        .betsizes
        .iter()
        .map(|betsize| build_data_row_with_boards(betsize, &size_dirs, &args))
        .unzip();

    (datarows, validate_identical_and_get(considered_boards))
}

fn get_size_dirs(positions: &Positions) -> Vec<DirEntry> {
    let pos_dir = get_pos_dir(DATA_DIR, positions);
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
            .all(|(action_name, action)| action_name == action.to_string())
}

fn get_lines_with_boards(file_content: &String) -> Vec<(&str, Board)> {
    let lines_with_boards = file_content
        .lines()
        .skip(1)
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

    Board::try_from(board_str).expect(&format!("Invalid board: {}", board_str))
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
    let heights_match = args.heights.iter().all(|height| board.is_height(&height));
    let suits_match = args.suits.iter().all(|suit| board.is_suit(&suit));
    let connections_match = args
        .connectednesses
        .iter()
        .all(|connection| board.is_connection(&connection));

    heights_match && suits_match && connections_match
}

fn build_data_row(lines: &Vec<String>) -> DataRow {
    let data_rows: Vec<DataRow> = lines.iter().map(|line| DataRow::new(line)).collect();
    let count = data_rows.len();

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
