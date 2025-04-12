use std::ops::{Add, Div};

use crate::{args::betsize::Betsize, poker::board::Board};

#[derive(Debug, PartialEq)]
pub struct DataRow {
    pub size: Option<Betsize>,
    pub eq: f32,
    pub ev: f32,
    pub bet_freq: f32,
    pub check_freq: f32,
}

impl DataRow {
    pub fn new(line: &str) -> DataRow {
        let split: Vec<&str> = line.split('\t').collect();

        assert!(split.len() >= 5);
        Board::try_from(split[0]).expect("Expected first part of line to be a valid board");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datarow_new() {
        let datarow = DataRow::new("8s8d8c	56.532	35.471	69.566	30.434");

        assert_eq!(datarow.eq, 56.532);
        assert_eq!(datarow.ev, 35.471);
        assert_eq!(datarow.bet_freq, 69.566);
        assert_eq!(datarow.check_freq, 30.434);
    }

    #[test]
    fn test_datarow_add() {
        let datarow1 = DataRow::new("8s8d8c	56.532	35.471	69.566	30.434");
        let datarow2 = DataRow::new("8s5s5d	54.398	28.831	6.355	93.645");
        let added_datarow = datarow1 + datarow2;

        assert_eq!(added_datarow.eq, 56.532 + 54.398);
        assert_eq!(added_datarow.ev, 35.471 + 28.831);
        assert_eq!(added_datarow.bet_freq, 69.566 + 6.355);
        assert_eq!(added_datarow.check_freq, 30.434 + 93.645);
    }

    #[test]
    fn test_datarow_div() {
        let datarow = DataRow::new("8s8d8c	110.93	64.302	75.921	124.079");
        let divided_by_2 = datarow / 2;

        assert_eq!(divided_by_2.eq, 110.93 / 2.0);
        assert_eq!(divided_by_2.ev, 64.302 / 2.0);
        assert_eq!(divided_by_2.bet_freq, 75.921 / 2.0);
        assert_eq!(divided_by_2.check_freq, 124.079 / 2.0);
    }
}
