use clap::Parser;

/// CLI configuration for the `octo-flow` application.
///
/// This struct defines the command-line interface and is parsed using
/// [`clap`](https://docs.rs/clap). It represents the user-provided options
/// that control how the program reads and filters GitHub event data.
///
/// # Fields
///
/// - `input`: Path to a newline-delimited JSON (NDJSON) file containing
///   GitHub events. Use `-` to read from standard input.
/// - `event`: Optional GitHub event type used to filter the stream.
///
/// # Examples
///
/// Read events from a file:
///
/// ```bash
/// octo-flow --input events.json
/// ```
///
/// Filter only `PushEvent` events:
///
/// ```bash
/// octo-flow --input events.json --event PushEvent
/// ```
///
/// Read from a pipeline:
///
/// ```bash
/// zcat 2026-03-11-15.json.gz | octo-flow --input - --event WatchEvent
/// ```
#[derive(Parser)]
#[clap(about)]
pub struct Config {
    /// Path to the NDJSON input file.
    ///
    /// This file should contain GitHub events in newline-delimited JSON format.
    /// If `-` is provided, the program reads from standard input (`stdin`).
    ///
    /// This enables streaming workflows such as:
    ///
    /// ```bash
    /// zcat events.json.gz | octo-flow --input -
    /// ```
    #[clap(long, help = "Path to the NDJSON file. Use - for stdin.")]
    pub input: String,

    /// Optional GitHub event type used to filter the stream.
    ///
    /// If provided, only events matching this type will be included in the
    /// output.
    ///
    /// Examples:
    ///
    /// - `PushEvent`
    /// - `PullRequestEvent`
    /// - `WatchEvent`
    /// - `ForkEvent`
    ///
    /// If omitted, all events are processed.
    #[clap(
        long,
        help = "The GitHub event type to filter (e.g., PushEvent, PullRequestEvent)."
    )]
    pub event: Option<String>,
}
