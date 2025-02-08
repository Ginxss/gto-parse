use std::env;

use crate::board::{
    connectedness::FlopConnectedness, flop_height::FlopHeight, suits::FlopSuitType,
};

// Position

#[derive(Clone)]
pub enum Position {
    LJ,
    HJ,
    CO,
    BTN,
    SB,
    BB,
}

impl Position {
    pub fn from_str(string: &str) -> Position {
        match string {
            "LJ" => Position::LJ,
            "HJ" => Position::HJ,
            "CO" => Position::CO,
            "BTN" => Position::BTN,
            "SB" => Position::SB,
            "BB" => Position::BB,
            other => panic!("Invalid position string: {}", other),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Position::LJ => "LJ",
            Position::HJ => "HJ",
            Position::CO => "CO",
            Position::BTN => "BTN",
            Position::SB => "SB",
            Position::BB => "BB",
        }
    }
}

pub struct Positions {
    pub ip: Position,
    pub oop: Position,
}

// Betsize

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Betsize {
    Size33,
    Size50,
    Size75,
    Size150,
}

impl Betsize {
    pub fn from_str(string: &str) -> Betsize {
        match string {
            "33" => Betsize::Size33,
            "50" => Betsize::Size50,
            "75" => Betsize::Size75,
            "150" => Betsize::Size150,
            other => panic!("Invalid betsize string: {}", other),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Betsize::Size33 => "33",
            Betsize::Size50 => "50",
            Betsize::Size75 => "75",
            Betsize::Size150 => "150",
        }
    }
}

// Action

pub enum Action {
    Check,
    Bet,
    Call,
    Raise,
    Fold,
}

impl Action {
    pub fn from_str(string: &str) -> Action {
        match string {
            "X" => Action::Check,
            "B" => Action::Bet,
            "C" => Action::Call,
            "R" => Action::Raise,
            "F" => Action::Fold,
            other => panic!("Invalid action string: {}", other),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            // TODO:
            Action::Check => "check",
            Action::Bet => "B",
            Action::Call => "C",
            Action::Raise => "R",
            Action::Fold => "F",
        }
    }
}

// Args

pub struct Args {
    pub positions: Positions,
    pub betsizes: Vec<Betsize>,
    pub heights: Vec<FlopHeight>,
    pub suits: Vec<FlopSuitType>,
    pub connectednesses: Vec<FlopConnectedness>,
    pub actions: Vec<Action>,
}

pub fn read_cmdline_args() -> Args {
    enum ParseMode {
        None,
        Positions,
        Betsizes,
        Heights,
        Suits,
        Connectednesses,
        Actions,
    }

    let mut positions: Vec<Position> = Vec::new();
    let mut betsizes: Vec<Betsize> = Vec::new();
    let mut heights: Vec<FlopHeight> = Vec::new();
    let mut suits: Vec<FlopSuitType> = Vec::new();
    let mut connectednesses: Vec<FlopConnectedness> = Vec::new();
    let mut actions: Vec<Action> = Vec::new();

    let mut curr_parse_mode: ParseMode = ParseMode::None;

    let args_uppercase = env::args().skip(1).map(|arg| arg.to_uppercase());
    for arg in args_uppercase {
        match &arg[..] {
            "-P" => {
                curr_parse_mode = ParseMode::Positions;
                continue;
            }
            "-B" => {
                curr_parse_mode = ParseMode::Betsizes;
                continue;
            }
            "-H" => {
                curr_parse_mode = ParseMode::Heights;
                continue;
            }
            "-S" => {
                curr_parse_mode = ParseMode::Suits;
                continue;
            }
            "-A" => {
                curr_parse_mode = ParseMode::Actions;
                continue;
            }

            token => match curr_parse_mode {
                ParseMode::Positions => positions.push(Position::from_str(token)),
                ParseMode::Betsizes => betsizes.push(Betsize::from_str(token)),
                ParseMode::Heights => heights.push(FlopHeight::from_str(token)),
                ParseMode::Suits => suits.push(FlopSuitType::from_str(token)),
                ParseMode::Connectednesses => {
                    connectednesses.push(FlopConnectedness::from_str(token))
                }
                ParseMode::Actions => actions.push(Action::from_str(token)),
                _ => panic!(),
            },
        }
    }

    assert!(positions.len() == 2);
    assert!(!actions.is_empty());

    let pos1 = positions.get(0).unwrap().to_owned();
    let pos2 = positions.get(1).unwrap().to_owned();
    let positions = Positions {
        ip: pos1,
        oop: pos2,
    };

    Args {
        positions,
        betsizes,
        heights,
        suits,
        connectednesses,
        actions,
    }
}
