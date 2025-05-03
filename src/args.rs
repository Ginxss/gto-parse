use std::{env, str::FromStr};

use crate::poker::{
    action::Action,
    betsize::Betsize,
    board::{connection::Connection, height::BoardHeight, pair::BoardPair, suit::BoardSuit},
    position::{Position, Positions},
};

pub struct Args {
    pub positions: Positions,
    pub betsizes: Vec<Betsize>,
    pub heights: Vec<BoardHeight>,
    pub suits: Vec<BoardSuit>,
    pub connections: Vec<Connection>,
    pub pair: Vec<BoardPair>,
    pub actions: Vec<Action>,
}

pub fn read_cmdline_args() -> Args {
    parse_args(env::args().skip(1))
}

fn parse_args(args: impl Iterator<Item = String>) -> Args {
    enum ParseMode {
        None,
        Positions,
        Betsizes,
        Heights,
        Suits,
        Connectednesses,
        Pair,
    }

    let mut positions: Vec<Position> = Vec::new();
    let mut betsizes: Vec<Betsize> = Vec::new();
    let mut heights: Vec<BoardHeight> = Vec::new();
    let mut suits: Vec<BoardSuit> = Vec::new();
    let mut connections: Vec<Connection> = Vec::new();
    let mut pair: Vec<BoardPair> = Vec::new();

    let mut curr_parse_mode: ParseMode = ParseMode::None;

    let args_uppercase = args.map(|arg| arg.to_uppercase());
    for arg in args_uppercase {
        match &arg[..] {
            "-PO" => {
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
            "-PA" => {
                curr_parse_mode = ParseMode::Pair;
                continue;
            }

            token => match curr_parse_mode {
                ParseMode::Positions => positions.push(Position::from_str(token).unwrap()),
                ParseMode::Betsizes => betsizes.push(Betsize::from_str(token).unwrap()),
                ParseMode::Heights => heights.push(BoardHeight::try_from(token).unwrap()),
                ParseMode::Suits => suits.push(BoardSuit::try_from(token).unwrap()),
                ParseMode::Connectednesses => {
                    connections.push(Connection::try_from(token).unwrap())
                }
                ParseMode::Pair => pair.push(BoardPair::from_str(token).unwrap()),
                _ => panic!(),
            },
        }
    }

    assert!(positions.len() == 2);

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
        connections,
        pair,
        // Only flop from the perspective of IP after OOP check is currently supported
        actions: vec![Action::Check],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        let args_vec = vec![
            "-po", "BTN", "BB", "-b", "33", "75", "150", "-h", "3BW", "2BW", "-s", "M", "-c", "DC",
            "GS", "OESD", "WH", "-pa", "U",
        ]
        .into_iter()
        .map(|arg| arg.to_string());

        let args = parse_args(args_vec);

        assert_eq!(args.positions.ip, Position::BTN);
        assert_eq!(args.positions.oop, Position::BB);

        assert_eq!(args.betsizes.len(), 3);
        assert!(args.betsizes.contains(&Betsize::Size33));
        assert!(args.betsizes.contains(&Betsize::Size75));
        assert!(args.betsizes.contains(&Betsize::Size150));

        assert_eq!(args.heights.len(), 2);
        assert!(args.heights.contains(&BoardHeight::TripleBW));
        assert!(args.heights.contains(&BoardHeight::DoubleBW));

        assert_eq!(args.suits.len(), 1);
        assert!(args.suits.contains(&BoardSuit::Montone));

        assert_eq!(args.connections.len(), 4);
        assert!(args.connections.contains(&Connection::Disconnected));
        assert!(args.connections.contains(&Connection::Gutshot));
        assert!(args.connections.contains(&Connection::OESD));
        assert!(args.connections.contains(&Connection::Wheel));

        assert_eq!(args.pair.len(), 1);
        assert!(args.pair.contains(&BoardPair::Unpaired));
    }
}
