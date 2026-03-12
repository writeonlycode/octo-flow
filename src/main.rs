//! Command-line interface for the `octo-flow` event processing tool.
//!
//! This binary acts as a thin wrapper around the `octo-flow` library.
//! It parses command-line arguments using `clap`, then invokes the
//! event processing pipeline implemented in the library.
//!
//! The CLI is designed to work well in Unix-style pipelines and with
//! large newline-delimited JSON (NDJSON) datasets such as those
//! published by GHArchive.

use anyhow::{Context, Result};
use clap::Parser;
use config::Config;
use octo_flow::run;

pub mod config;

/// Application entry point.
///
/// This function:
///
/// 1. Parses command-line arguments.
/// 2. Runs the `octo-flow` event processing pipeline.
/// 3. Adds contextual error information if the pipeline fails.
///
/// Errors are returned using `anyhow::Result`, which provides flexible
/// error handling and allows additional context to be attached to failures.
///
/// # Errors
///
/// Returns an error if:
///
/// - CLI arguments cannot be parsed
/// - the input file cannot be opened
/// - an I/O error occurs while reading
/// - JSON event parsing fails
fn main() -> Result<()> {
    let config = Config::parse();

    run(config.input, config.event)
        .context("The octo-flow engine encountered a critical error during processing.")?;

    Ok(())
}
