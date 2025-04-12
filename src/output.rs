use prettytable::{format, row, Row, Table};

use crate::{calculation::datarow::DataRow, poker::board::Board};

pub fn print_result(data_rows: Vec<DataRow>, boards: Vec<Board>) {
    println!("Considered boards: {:#?}\n", boards);
    print_table(data_rows);
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
