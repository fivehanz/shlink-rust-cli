mod api;
mod commands;
mod config;
mod utils;

use clap::Parser;
use commands::Cli;

fn main() {
    let _cli = Cli::parse();
}
