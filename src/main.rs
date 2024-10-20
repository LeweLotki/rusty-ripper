pub mod cli;
use cli::parser::CLI;

fn main() {
    CLI::run_parser();
}
