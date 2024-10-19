use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
    vec,
};

use prettytable::{row, Table};

mod board_utils;

type Betsize = String;
type Positions = (String, String);

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

#[derive(Debug, PartialEq)]
enum Category {
    SingleBW,
    DoubleBW,
    TripleBW,
    Middling,
    Low,
}

#[derive(Debug)]
enum Modifier {
    Rainbow,
    Twotone,
    Montone,
    Disconnected,
    TwoConnected,
    ThreeConnected,
}

struct Args {
    categories: Vec<Category>,
    modifiers: Vec<Modifier>,
    betsizes: Vec<Betsize>,
    positions: Positions,
    actions: Vec<String>,
}

fn main() {
    let Args {
        categories,
        modifiers,
        betsizes,
        positions,
        actions,
    } = read_cmd_args();

    let datas = build_datas(&positions, &betsizes, &actions, &categories);

    print_table(&datas);
}

fn read_cmd_args() -> Args {
    enum ParseMode {
        None,
        Categories,
        Modifiers,
        Betsizes,
        Positions,
        ActionSequence,
    }

    let mut categories: Vec<Category> = Vec::new();
    let mut modifiers: Vec<Modifier> = Vec::new();
    let mut betsizes: Vec<Betsize> = Vec::new();
    let mut positions: Vec<String> = Vec::new();
    let mut actions: Vec<String> = Vec::new();

    let mut curr_parse_mode: ParseMode = ParseMode::None;

    let args_lowercase = env::args().skip(1).map(|arg| arg.to_lowercase());
    for arg in args_lowercase {
        match &arg[..] {
            "-c" => {
                curr_parse_mode = ParseMode::Categories;
                continue;
            }
            "-m" => {
                curr_parse_mode = ParseMode::Modifiers;
                continue;
            }
            "-b" => {
                curr_parse_mode = ParseMode::Betsizes;
                continue;
            }
            "-p" => {
                curr_parse_mode = ParseMode::Positions;
                continue;
            }
            "-a" => {
                curr_parse_mode = ParseMode::ActionSequence;
                continue;
            }

            token => match curr_parse_mode {
                ParseMode::Categories => match token {
                    "1bw" => categories.push(Category::SingleBW),
                    "2bw" => categories.push(Category::DoubleBW),
                    "3bw" => categories.push(Category::TripleBW),
                    "mid" | "middling" => categories.push(Category::Middling),
                    "low" => categories.push(Category::Low),
                    _ => panic!(),
                },
                ParseMode::Modifiers => match token {
                    "r" | "rb" => modifiers.push(Modifier::Rainbow),
                    "t" | "tt" => modifiers.push(Modifier::Twotone),
                    "m" | "mt" => modifiers.push(Modifier::Montone),
                    "dc" => modifiers.push(Modifier::Disconnected),
                    "2c" => modifiers.push(Modifier::TwoConnected),
                    "3c" => modifiers.push(Modifier::ThreeConnected),
                    _ => panic!(),
                },
                ParseMode::Betsizes => match token.parse::<i32>() {
                    Ok(_) => betsizes.push(arg),
                    Err(_) => panic!(),
                },
                ParseMode::Positions => positions.push(arg),
                ParseMode::ActionSequence => actions.push(arg),
                _ => panic!(),
            },
        }
    }

    assert!(positions.len() == 2);
    assert!(!actions.is_empty());

    let pos1 = positions.get(0).unwrap().to_owned();
    let pos2 = positions.get(1).unwrap().to_owned();

    Args {
        categories,
        modifiers,
        betsizes,
        positions: (pos1, pos2),
        actions,
    }
}

fn build_datas(
    positions: &Positions,
    betsizes: &Vec<Betsize>,
    actions: &Vec<String>,
    categories: &Vec<Category>,
) -> Vec<Data> {
    let pos_dir = get_valid_child_dirs(PathBuf::from(DATA_DIR))
        .find(|dir| {
            let name = get_filename(dir).to_lowercase();
            name.contains(&positions.0) && name.contains(&positions.1)
        })
        .expect("Could not find position directory");

    let collect_1bw = categories.contains(&Category::SingleBW);
    let collect_2bw = categories.contains(&Category::DoubleBW);
    let collect_3bw = categories.contains(&Category::TripleBW);
    let collect_mid = categories.contains(&Category::Middling);
    let collect_low = categories.contains(&Category::Low);

    let datas: Vec<Data> = betsizes
        .iter()
        .map(|size| {
            let size_dir = get_valid_child_dirs(pos_dir.path())
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
                .filter(|data| {
                    collect_1bw && board_utils::is_1bw(&data.board)
                        || collect_2bw && board_utils::is_2bw(&data.board)
                        || collect_3bw && board_utils::is_3bw(&data.board)
                        || collect_mid && board_utils::is_middling(&data.board)
                        || collect_low && board_utils::is_low(&data.board)
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
    get_valid_child_files(size_dir.path()).find(|file| match file.path().file_stem() {
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

fn get_valid_child_dirs(path: PathBuf) -> impl Iterator<Item = DirEntry> {
    fs::read_dir(&path)
        .expect(&format!("Error reading directory: {:#?}", path))
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().is_dir() && !entry.file_name().to_string_lossy().starts_with(".")
        })
}

fn get_valid_child_files(path: PathBuf) -> impl Iterator<Item = DirEntry> {
    fs::read_dir(&path)
        .expect(&format!("Error reading directory: {:#?}", path))
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().is_file() && !entry.file_name().to_string_lossy().starts_with(".")
        })
}
