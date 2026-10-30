use std::process::exit;

use crate::cli::check_args;

mod build_file;
mod cli;
mod helper;
mod tui;

fn main() {
    check_args().unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1)
    });
}
