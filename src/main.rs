use clap::Parser;
use octo_flow::{config::Config, run};
use std::process::exit;

fn main() {
    let config = Config::parse();

    if let Err(error) = run(config) {
        eprintln!("Ops, something went wrong: {error}");
        exit(1);
    }
}
