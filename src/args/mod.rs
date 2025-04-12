pub mod action;
pub mod betsize;
pub mod position;

use std::{env, str::FromStr};

use action::Action;
use betsize::Betsize;
use position::{Position, Positions};

use crate::poker::board::{connection::BoardConnection, height::BoardHeight, suit::BoardSuit};

pub struct Args {
    pub positions: Positions,
    pub betsizes: Vec<Betsize>,
    pub heights: Vec<BoardHeight>,
    pub suits: Vec<BoardSuit>,
    pub connectednesses: Vec<BoardConnection>,
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
    let mut heights: Vec<BoardHeight> = Vec::new();
    let mut suits: Vec<BoardSuit> = Vec::new();
    let mut connections: Vec<BoardConnection> = Vec::new();
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
            "-C" => {
                curr_parse_mode = ParseMode::Connectednesses;
                continue;
            }
            "-A" => {
                curr_parse_mode = ParseMode::Actions;
                continue;
            }

            token => match curr_parse_mode {
                ParseMode::Positions => {
                    positions.push(Position::from_str(token).expect("error parsing position"))
                }
                ParseMode::Betsizes => {
                    betsizes.push(Betsize::from_str(token).expect("error parsing betsize"))
                }
                ParseMode::Heights => heights.push(BoardHeight::try_from(token).unwrap()),
                ParseMode::Suits => suits.push(BoardSuit::try_from(token).unwrap()),
                ParseMode::Connectednesses => {
                    connections.push(BoardConnection::try_from(token).unwrap())
                }
                ParseMode::Actions => {
                    actions.push(Action::from_str(token).expect("error parsing action"))
                }
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
        connectednesses: connections,
        actions,
    }
}
