mod args;
mod calculation;
mod files;
mod output;
mod poker;

fn main() {
    let args = args::read_cmdline_args();

    let (data_rows, boards) = calculation::build_data_rows_with_boards(args);

    output::print_result(data_rows, boards);
}
