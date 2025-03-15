/*
 * TODO:
 * - Paired Boards
 * - Duplicate checks
 * - Show considered boards
 */

mod args;
mod board;
mod datarow;
mod files;

use std::{
    fs::{self, DirEntry},
    path::Path,
    vec,
};

use args::{action::Action, betsize::Betsize, position::Positions, Args};
use board::{connectedness::is_connectedness, flop_height::is_height, suits::is_suit_type};
use datarow::DataRow;
use prettytable::{format, row, table, Row};

fn main() {
    let args = args::read_cmdline_args();

    let data_rows = build_data_rows(&args);

    print_table(&data_rows);
}

fn build_data_rows(args: &Args) -> Vec<DataRow> {
    let size_dirs = get_size_dirs(&args.positions);

    args.betsizes
        .iter()
        .map(|betsize| to_data_row(betsize, &size_dirs, &args))
        .collect()
}

fn get_size_dirs(positions: &Positions) -> Vec<DirEntry> {
    let pos_dir = get_pos_dir("./data", positions);
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

fn to_data_row(betsize: &Betsize, size_dirs: &Vec<DirEntry>, args: &Args) -> DataRow {
    let size_dir = size_dirs
        .iter()
        .find(|dir| files::get_name(&dir.path()) == betsize.to_string())
        .expect(&format!("Could not find size dir for {betsize}"));

    let action_file = get_action_file_in_dir(size_dir, &args.actions);
    let file_content = fs::read_to_string(action_file.path()).expect("Could not read file content");

    let filteres_lines: Vec<String> = file_content
        .lines()
        .skip(1)
        .filter(|line| line_matches_conditions(line, &args))
        .map(|s| s.to_string())
        .collect();

    let mut data_row = build_data_row(&filteres_lines);
    data_row.size = Some(betsize.clone());
    data_row
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

fn line_matches_conditions(line: &str, args: &Args) -> bool {
    let board = line
        .split('\t')
        .next()
        .expect("Error getting board from line");

    let match_height =
        args.heights.is_empty() || args.heights.iter().any(|height| is_height(board, height));

    let match_suits =
        args.suits.is_empty() || args.suits.iter().any(|suit| is_suit_type(board, suit));

    let match_connectedness = args.connectednesses.is_empty()
        || args
            .connectednesses
            .iter()
            .any(|connectedness| is_connectedness(board, connectedness));

    match_height && match_suits && match_connectedness
}

fn build_data_row(lines: &Vec<String>) -> DataRow {
    let data_rows: Vec<DataRow> = lines
        .iter()
        .skip(1)
        .map(|line| DataRow::new(line))
        .collect();

    let count = data_rows.len();

    data_rows
        .into_iter()
        .reduce(|row1, row2| row1 + row2)
        .map(|sum_row| sum_row / count)
        .expect("Could not calculate data row")
}

fn print_table(data_rows: &Vec<DataRow>) {
    let max_row = data_rows
        .iter()
        .max_by(|row1, row2| {
            row1.ev
                .partial_cmp(&row2.ev)
                .expect(&format!("Could not compare {} and {}", row1.ev, row2.ev))
        })
        .expect("could not determine size of max row");

    let mut table = table!();
    table.set_titles(row!["Size", "EQ", "EV", "Bet", "Check", "EV Difference"]);
    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    data_rows
        .iter()
        .map(|row| build_table_row(row, max_row))
        .for_each(|row| {
            table.add_row(row);
        });

    table.printstd();
}

fn build_table_row(row: &DataRow, max_row: &DataRow) -> Row {
    let size_str = row
        .size
        .as_ref()
        .map(|size| size.to_string())
        .unwrap_or(String::from("-"));

    let ev_diff = row.ev - max_row.ev;
    let bb_per_100 = ev_diff * 10.0;
    let ev_diff_str = format!("{ev_diff:.2} = {bb_per_100:.1} BB/100");

    let eq_str = format!("{:.2}", row.eq);
    let ev_str = format!("{:.2}", row.ev);
    let bet_str = format!("{:.2}", row.bet_freq);
    let check_str = format!("{:.2}", row.check_freq);

    if row == max_row {
        row![b->size_str, b->eq_str, b->ev_str, bFR->bet_str, bFG->check_str, b->ev_diff]
    } else {
        row![size_str, eq_str, ev_str, bet_str, check_str, Fr->ev_diff_str]
    }
}
