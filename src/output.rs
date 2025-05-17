use prettytable::{format, row, Row, Table};

use crate::{calculation::datarow::DataRow, poker::board::Board};

pub fn print_result(data_rows: Vec<DataRow>, boards: Vec<Board>) {
    print_considered_board(boards);
    print_table(data_rows);
}

fn print_considered_board(boards: Vec<Board>) {
    let joined_boards = boards
        .iter()
        .map(|board| board.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!("Considered boards: {}", joined_boards);
}

fn print_table(data_rows: Vec<DataRow>) {
    let max_row = get_max_ev_row(&data_rows);

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(row!["Size", "EQ", "EV", "Bet", "Check", "EV Difference"]);

    data_rows
        .iter()
        .map(|row| build_table_row(row, max_row))
        .for_each(|row| {
            table.add_row(row);
        });

    table.printstd();
}

fn get_max_ev_row(data_rows: &Vec<DataRow>) -> &DataRow {
    data_rows
        .iter()
        .max_by(|row1, row2| {
            row1.ev
                .partial_cmp(&row2.ev)
                .expect(&format!("Could not compare {} and {}", row1.ev, row2.ev))
        })
        .expect("could not determine size of max row")
}

fn build_table_row(row: &DataRow, max_row: &DataRow) -> Row {
    let size_str = row
        .size
        .as_ref()
        .map(|size| size.to_string())
        .unwrap_or(String::from("-"));

    let eq_str = format!("{:.2}", row.eq);
    let ev_str = format!("{:.2}", row.ev);
    let bet_str = format!("{:.2}", row.bet_freq);
    let check_str = format!("{:.2}", row.check_freq);

    let ev_diff = row.ev - max_row.ev;
    let bb_per_100 = ev_diff * 10.0;
    let ev_diff_str = format!("{ev_diff:.2} = {bb_per_100:.1} BB/100");

    if row == max_row {
        row![b->size_str, b->eq_str, b->ev_str, bFR->bet_str, bFG->check_str, b->ev_diff]
    } else {
        row![size_str, eq_str, ev_str, bet_str, check_str, Fr->ev_diff_str]
    }
}

#[cfg(test)]
mod tests {
    use crate::poker::betsize::Betsize;

    use super::*;

    #[test]
    fn test_get_max_ev_row() {
        let data_rows = vec![
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

        let max_row = get_max_ev_row(&data_rows);

        assert_eq!(*max_row.size.as_ref().unwrap(), Betsize::Size50);
    }

    #[test]
    fn test_build_table_row() {
        let data_rows = vec![
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

        let max_row = DataRow {
            size: Some(Betsize::Size50),
            eq: 52.90833,
            ev: 41.9065,
            bet_freq: 28.626001,
            check_freq: 75.374504,
        };

        let table_rows: Vec<_> = data_rows
            .iter()
            .map(|row| build_table_row(row, &max_row))
            .collect();

        let expected_table_row_33 =
            row!["33", "60.76", "28.56", "28.13", "65.88", Fr->"-13.35 = -133.5 BB/100"];
        let expected_table_row_50 =
            row![b->"50", b->"52.91", b->"41.91", bFR->"28.63", bFG->"75.37", b->"0"];
        let expected_table_row_75 =
            row!["75", "61.08", "39.07", "21.63", "69.04", Fr->"-2.83 = -28.3 BB/100"];
        let expected_table_row_150 =
            row!["150", "55.74", "31.74", "27.46", "71.71", Fr->"-10.17 = -101.7 BB/100"];

        assert_eq!(table_rows.len(), 4);
        assert!(table_rows.contains(&expected_table_row_33));
        assert!(table_rows.contains(&expected_table_row_50));
        assert!(table_rows.contains(&expected_table_row_75));
        assert!(table_rows.contains(&expected_table_row_150));
    }
}
