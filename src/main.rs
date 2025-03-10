/*
 * TODO:
 * - Color Code Table
 * - Paired Boards
 * - Duplicate checks
 * - Show considered boards
 */

mod args;
mod board;
mod files;

use std::{
    fs::{self, DirEntry},
    ops::{Add, Div},
    path::Path,
    vec,
};

use args::{action::Action, betsize::Betsize, position::Positions, Args};
use board::{
    connectedness::{is_connectedness, FlopConnectedness},
    flop_height::{is_height, FlopHeight},
    suits::{is_suit_type, FlopSuitType},
};
use prettytable::{row, Table};

#[derive(Debug)]
struct DataRow {
    size: Option<Betsize>,
    eq: f32,
    ev: f32,
    bet_freq: f32,
    check_freq: f32,
}

impl DataRow {
    fn new(line: &str) -> DataRow {
        let split: Vec<&str> = line.split('\t').collect();
        assert!(split.len() >= 5);

        DataRow {
            size: None,
            eq: split[1].parse().expect("Equity needs to be a number"),
            ev: split[2].parse().expect("EV needs to be a number"),
            bet_freq: split[3].parse().expect("Bet freq. needs to be a number"),
            check_freq: split[4].parse().expect("Check freq. needs to be a number"),
        }
    }
}

impl Add for DataRow {
    type Output = DataRow;

    fn add(self, other: DataRow) -> DataRow {
        assert_eq!(self.size, other.size);

        DataRow {
            size: self.size,
            eq: self.eq + other.eq,
            ev: self.ev + other.ev,
            bet_freq: self.bet_freq + other.bet_freq,
            check_freq: self.check_freq + other.check_freq,
        }
    }
}

impl Div<usize> for DataRow {
    type Output = DataRow;

    fn div(self, divisor: usize) -> DataRow {
        assert_ne!(divisor, 0);

        let divisor = divisor as f32;

        DataRow {
            size: self.size,
            eq: self.eq / divisor,
            ev: self.ev / divisor,
            bet_freq: self.bet_freq / divisor,
            check_freq: self.check_freq / divisor,
        }
    }
}

fn main() {
    let args = args::read_cmdline_args();

    let pos_dir = get_pos_dir("./data", &args.positions);

    let size_dirs = files::get_dirs(&pos_dir.path());

    let data_rows = to_data_rows(&size_dirs, &args);

    print_table(&data_rows);
}

fn to_data_rows(size_dirs: &Vec<DirEntry>, args: &Args) -> Vec<DataRow> {
    args.betsizes
        .iter()
        .map(|betsize| to_data_row(betsize, &size_dirs, &args))
        .collect()
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
        .filter(|line| {
            line_matches_conditions(line, &args.heights, &args.suits, &args.connectednesses)
        })
        .map(|s| s.to_string())
        .collect();

    let mut data_row = build_data_row(&filteres_lines);
    data_row.size = Some(betsize.clone());
    data_row
}

fn line_matches_conditions(
    line: &str,
    heights: &Vec<FlopHeight>,
    suits: &Vec<FlopSuitType>,
    connectednesses: &Vec<FlopConnectedness>,
) -> bool {
    let board = line
        .split('\t')
        .next()
        .expect("Error getting board from line");

    let match_height = heights.is_empty() || heights.iter().any(|height| is_height(board, height));

    let match_suits = suits.is_empty() || suits.iter().any(|suit| is_suit_type(board, suit));

    let match_connectedness = connectednesses.is_empty()
        || connectednesses
            .iter()
            .any(|connectedness| is_connectedness(board, connectedness));

    match_height && match_suits && match_connectedness
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
    let mut table = Table::new();

    table.add_row(row!["Size", "EQ", "EV", "Bet", "Check"]);

    data_rows
        .iter()
        .map(|data| {
            let size = data
                .size
                .as_ref()
                .map(|size| size.to_string())
                .unwrap_or("-".to_string());

            row![size, data.eq, data.ev, data.bet_freq, data.check_freq]
        })
        .for_each(|row| {
            table.add_row(row);
        });

    table.printstd();
}
