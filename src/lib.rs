use crate::{config::Config, github_event::GitHubEvent};
use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

pub mod config;
pub mod github_event;

pub fn run(config: Config) -> Result<()> {
    let input_source: Box<dyn Read> = if config.input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(config.input)?)
    };

    let reader = BufReader::new(input_source);

    for line_result in reader.lines() {
        let line = line_result?;
        let github_event: GitHubEvent = serde_json::from_str(&line)?;

        if config
            .event
            .as_deref()
            .map_or(true, |f| f == github_event.kind)
        {
            println!(
                "{}\t{}\t{}\t{}\t{}",
                github_event.id,
                github_event.created_at,
                github_event.actor.display_login,
                github_event.repo.name.unwrap_or("n/a"),
                github_event.kind
            )
        }
    }

    Ok(())
}
