pub mod cli;
pub mod modes;

use cli::parser::CLI;

fn main() {
    CLI::run_parser();
}
