use clap::Parser;

#[derive(Parser)]
#[clap(about)]
pub struct Config {
    #[clap(long, help = "Path to the NDJSON file. Use - for stdin.")]
    pub input: String,

    #[clap(
        long,
        help = "The GitHub event type to filter (e.g., PushEvent, PullRequestEvent)."
    )]
    pub event: Option<String>,
}
