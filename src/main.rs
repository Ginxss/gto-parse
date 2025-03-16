/*
 * TODO:
 * - Paired Boards
 * - Show considered boards
 */

mod args;
mod board;
mod calculation;
mod files;
mod table;

fn main() {
    let args = args::read_cmdline_args();

    let data_rows = calculation::build_data_rows(args);

    table::print_table(data_rows);
}
