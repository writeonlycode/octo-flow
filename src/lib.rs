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

pub fn run(input: String, event: Option<String>) -> Result<usize, OctoFlowError> {
    let input_source: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    process_events(input_source, event)
}

pub fn process_events<R: Read>(source: R, event: Option<String>) -> Result<usize, OctoFlowError> {
    let reader = BufReader::new(source);
    let mut count = 0;

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
            );

            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_event_parsing() {
        let data = r#"{"id":"123","type":"PushEvent","actor":{"login":"coder"},"repo":{"name":"rust-lang/rust"},"created_at":"2026-03-12"}"#;
        let reader = data.as_bytes();

        let result = process_events(reader, None);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_filter_works() {
        let data = "{\"id\":\"1\",\"type\":\"PushEvent\",\"actor\":{\"login\":\"a\"},\"repo\":{\"name\":\"r\"},\"created_at\":\"d\"}\n\
                    {\"id\":\"2\",\"type\":\"WatchEvent\",\"actor\":{\"login\":\"b\"},\"repo\":{\"name\":\"r\"},\"created_at\":\"d\"}";
        let reader = data.as_bytes();

        let result = process_events(reader, Some("WatchEvent".to_string())).unwrap();

        assert_eq!(result, 1);
    }
}
