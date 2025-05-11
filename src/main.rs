mod args;
mod calculation;
mod files;
mod output;
mod poker;

// TODO: real looking test data and then also add tests for the calculation -> and then it's done!
fn main() {
    let args = args::read_cmdline_args();

    let (data_rows, boards) = calculation::build_data_rows_with_boards(args);

    output::print_result(data_rows, boards);
}
