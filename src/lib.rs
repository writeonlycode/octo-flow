use crate::github_event::GitHubEvent;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};
use thiserror::Error;

pub mod github_event;

#[derive(Debug, Error)]
pub enum OctoFlowError {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    ParseError(#[from] serde_json::Error),
}

pub fn run(input: String, event: Option<String>) -> Result<(), OctoFlowError> {
    let input_source: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let reader = BufReader::new(input_source);

    for line_result in reader.lines() {
        let line = line_result?;
        let github_event: GitHubEvent = serde_json::from_str(&line)?;

        if event.as_deref().map_or(true, |f| f == github_event.kind) {
            println!(
                "{}\t{}\t{}\t{}\t{}",
                github_event.id,
                github_event.created_at,
                github_event.actor.login.unwrap_or("n/a"),
                github_event.repo.name.unwrap_or("n/a"),
                github_event.kind
            )
        }
    }

    Ok(())
}
