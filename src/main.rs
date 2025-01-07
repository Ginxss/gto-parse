/*
 * TODO:
 * - Color Code Table
 * - Data Add cleanup
 * - Paired Boards (as modifier?)
 * - Duplicate checks
 */

mod args;
mod board;
mod files;

use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
    vec,
};

use prettytable::{row, Table};

use board::{
    flop_height::{is_1bw, is_2bw, is_3bw, is_low, is_middling},
    suits::{is_monotone, is_rainbow, is_twotone},
};

const DATA_DIR: &str = "./data";

#[derive(Debug)]
struct Data {
    board: String,
    size: Betsize,
    eq: f32,
    ev: f32,
    bet_freq: f32,
    check_freq: f32,
}

fn main() {
    let Args {
        categories,
        modifiers,
        betsizes,
        positions,
        actions,
    } = read_cmd_args();

    let datas = build_datas(&positions, &betsizes, &actions, &categories, &modifiers);

    print_table(&datas);
}

fn build_datas(
    positions: &Positions,
    betsizes: &Vec<Betsize>,
    actions: &Vec<String>,
    categories: &Vec<BoardCategory>,
    modifiers: &Vec<Modifier>,
) -> Vec<Data> {
    let pos_dir = files::get_valid_child_dirs(PathBuf::from(DATA_DIR))
        .find(|dir| {
            let name = get_filename(dir).to_lowercase();
            name.contains(&positions.0) && name.contains(&positions.1)
        })
        .expect("Could not find position directory");

    let collect_1bw = categories.contains(&BoardCategory::SingleBW);
    let collect_2bw = categories.contains(&BoardCategory::DoubleBW);
    let collect_3bw = categories.contains(&BoardCategory::TripleBW);
    let collect_mid = categories.contains(&BoardCategory::Middling);
    let collect_low = categories.contains(&BoardCategory::Low);

    let collect_rb = modifiers.contains(&Modifier::Rainbow);
    let collect_tt = modifiers.contains(&Modifier::Twotone);
    let collect_mono = modifiers.contains(&Modifier::Montone);

    let datas: Vec<Data> = betsizes
        .iter()
        .map(|size| {
            let size_dir = files::get_valid_child_dirs(pos_dir.path())
                .find(|size_dir| get_filename(&size_dir) == *size)
                .expect("Could not find size directory");

            let action_file =
                find_action_file(&size_dir, actions).expect("Could not find action file");

            let file_content =
                fs::read_to_string(action_file.path()).expect("Could not read file content");

            let datas = file_content
                .lines()
                .skip(1)
                .map(|line| build_data_from_line(line, size))
                .filter(|data| match categories.len() {
                    0 => true,
                    _ => {
                        collect_1bw && is_1bw(&data.board)
                            || collect_2bw && is_2bw(&data.board)
                            || collect_3bw && is_3bw(&data.board)
                            || collect_mid && is_middling(&data.board)
                            || collect_low && is_low(&data.board)
                    }
                })
                .filter(|data| match modifiers.len() {
                    0 => true,
                    _ => {
                        collect_rb && is_rainbow(&data.board)
                            || collect_tt && is_twotone(&data.board)
                            || collect_mono && is_monotone(&data.board)
                    }
                });

            let count = datas.clone().count() as f32;

            let calculated = datas
                .reduce(|data1, data2| Data {
                    board: data1.board,
                    size: data1.size,
                    eq: data1.eq + data2.eq,
                    ev: data1.ev + data2.ev,
                    bet_freq: data1.bet_freq + data2.bet_freq,
                    check_freq: data1.check_freq + data2.check_freq,
                })
                .map(|added_data| Data {
                    board: added_data.board,
                    size: added_data.size,
                    eq: added_data.eq / count,
                    ev: added_data.ev / count,
                    bet_freq: added_data.bet_freq / count,
                    check_freq: added_data.check_freq / count,
                })
                .expect("Could not calculate results");

            calculated
        })
        .collect();

    assert!(betsizes.len() == datas.len());

    datas
}

fn print_table(datas: &Vec<Data>) {
    let mut table = Table::new();

    table.add_row(row!["Size", "EQ", "EV", "Bet", "Check"]);

    datas
        .iter()
        .map(|data| row![data.size, data.eq, data.ev, data.bet_freq, data.check_freq])
        .for_each(|row| {
            table.add_row(row);
        });

    table.printstd();
}

fn get_filename(entry: &DirEntry) -> String {
    let os_name = entry.file_name();
    return os_name.to_string_lossy().to_lowercase();
}

fn find_action_file(size_dir: &DirEntry, line: &Vec<String>) -> Option<DirEntry> {
    files::get_valid_child_files(size_dir.path()).find(|file| match file.path().file_stem() {
        Some(name) => {
            let split_pattern = '_';
            let name = name.to_string_lossy();

            return name.matches(split_pattern).count() == line.len()
                && name
                    .split(split_pattern)
                    .skip(1)
                    .zip(line.iter())
                    .all(|(name_part, action)| name_part == action.as_str());
        }
        None => false,
    })
}

fn build_data_from_line(line: &str, size: &str) -> Data {
    let split: Vec<&str> = line.split('\t').collect();
    assert!(split.len() >= 5);

    Data {
        board: split[0].to_string(),
        size: size.to_string(),
        eq: split[1].parse().expect("Equity needs to be a number"),
        ev: split[2].parse().expect("EV needs to be a number"),
        bet_freq: split[3].parse().expect("Bet freq. needs to be a number"),
        check_freq: split[4].parse().expect("Check freq. needs to be a number"),
    }
}
