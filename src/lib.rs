//! Core library for the `octo-flow` GitHub event processing pipeline.
//!
//! `octo-flow` is a streaming CLI tool for processing **GitHub Archive
//! (GHArchive) event datasets**, which are distributed as
//! newline-delimited JSON (NDJSON).
//!
//! The library provides the core event-processing pipeline used by the CLI.
//! It reads events from an input source, parses them using `serde_json`,
//! optionally filters them by event type, and outputs selected fields
//! in a tab-separated format.
//!
//! # Architecture
//!
//! The processing pipeline follows a streaming architecture designed to
//! handle large datasets efficiently:
//!
//! ```text
//! input source (file or stdin)
//!        ↓
//!     BufReader
//!        ↓
//!   line-by-line iterator
//!        ↓
//!   serde_json parsing
//!        ↓
//!   optional event filtering
//!        ↓
//!   tab-separated output
//! ```
//!
//! This approach ensures:
//!
//! - **constant memory usage**
//! - **fast startup time**
//! - **efficient processing of large NDJSON datasets**
//!
//! # Example
//!
//! ```no_run
//! use octo_flow::run;
//!
//! // Process a GitHub event file without filtering
//! run("events.json".to_string(), None).unwrap();
//!
//! // Process only PushEvent events
//! run("events.json".to_string(), Some("PushEvent".to_string())).unwrap();
//! ```
//!
//! # CLI Usage
//!
//! ```bash
//! octo-flow --input events.json --event PushEvent
//! ```

use crate::github_event::GitHubEvent;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};
use thiserror::Error;

pub mod github_event;

/// Error type for the `octo-flow` processing pipeline.
///
/// This enum represents all possible errors that can occur while
/// reading and processing GitHub event streams.
#[derive(Debug, Error)]
pub enum OctoFlowError {
    /// Error while reading from the input source.
    #[error(transparent)]
    IoError(#[from] io::Error),

    /// Error while parsing JSON event data.
    #[error(transparent)]
    ParseError(#[from] serde_json::Error),
}

/// Entry point for the event processing pipeline.
///
/// This function determines the input source and delegates processing
/// to [`process_events`].
///
/// # Parameters
///
/// - `input` — Path to the NDJSON input file. Use `-` to read from `stdin`.
/// - `event` — Optional GitHub event type filter.
///
/// # Returns
///
/// Returns the number of events that matched the filter and were printed.
///
/// # Errors
///
/// Returns an [`OctoFlowError`] if:
///
/// - the input file cannot be opened
/// - an I/O error occurs while reading
/// - a JSON parsing error occurs
///
/// # Example
///
/// ```no_run
/// use octo_flow::run;
///
/// run("events.json".to_string(), None).unwrap();
/// ```
pub fn run(input: String, event: Option<String>) -> Result<usize, OctoFlowError> {
    let input_source: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    process_events(input_source, event)
}

/// Process a stream of GitHub events.
///
/// This function reads newline-delimited JSON events from the provided
/// input source, parses each event, optionally filters by event type,
/// and prints selected event fields in **tab-separated format**.
///
/// The function processes events **line-by-line**, which enables efficient
/// handling of large datasets without loading the entire file into memory.
///
/// # Parameters
///
/// - `source` — Input stream containing NDJSON event data.
/// - `event` — Optional event type used to filter events.
///
/// # Output Format
///
/// Matching events are printed to standard output using the following format:
///
/// ```text
/// id    created_at    actor_login    repo_name    event_type
/// ```
///
/// Missing fields are replaced with `"n/a"`.
///
/// # Returns
///
/// Returns the number of events printed.
///
/// # Errors
///
/// Returns an [`OctoFlowError`] if:
///
/// - reading from the input source fails
/// - JSON parsing fails
///
/// # Example
///
/// ```no_run
/// use octo_flow::process_events;
///
/// let data = r#"{"id":"1","type":"PushEvent","actor":{"login":"a"},"repo":{"name":"r"},"created_at":"d"}"#;
/// let reader = data.as_bytes();
///
/// let count = process_events(reader, None).unwrap();
///
/// assert_eq!(count, 1);
/// ```
pub fn process_events<R: Read>(source: R, event: Option<String>) -> Result<usize, OctoFlowError> {
    let reader = BufReader::new(source);
    let mut count = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let github_event: GitHubEvent = serde_json::from_str(&line)?;

        if event.as_deref().is_none_or(|f| f == github_event.kind) {
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
    fn test_valid_event_parsing_works() {
        let data = r#"{"id":"123","type":"PushEvent","actor":{"login":"coder"},"repo":{"name":"rust-lang/rust"},"created_at":"2026-03-12"}"#;
        let reader = data.as_bytes();

        let result = process_events(reader, None);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_invalid_event_parsing_fails() {
        let data = r#"{"id":"123","type":"PushEvent","actor":{"login":"coder"},"repo":{"name":"rust-lang/rust"}"created_at":"2026-03-12"}"#;
        let reader = data.as_bytes();

        let result = process_events(reader, None);

        assert!(result.is_err());
        assert!(matches!(result, Err(OctoFlowError::ParseError(_))))
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
