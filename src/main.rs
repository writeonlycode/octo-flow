use anyhow::{Context, Result};
use clap::Parser;
use config::Config;
use octo_flow::run;

pub mod config;

fn main() -> Result<()> {
    let config = Config::parse();
    run(config.input, config.event)
        .context("The octo-flow engine encountered a critical error during processing.")?;
    Ok(())
}
