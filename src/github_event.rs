use serde::{Deserialize, Serialize};

/// Representation of a GitHub event from the GHArchive dataset.
///
/// GitHub Archive provides event streams in **newline-delimited JSON (NDJSON)**
/// format. Each line represents a single GitHub event.
///
/// This struct models the subset of fields used by `octo-flow` when processing
/// the event stream.
///
/// # Zero-Copy Deserialization
///
/// The struct uses **borrowed string slices (`&str`)** instead of owned
/// `String`s. This allows Serde to deserialize directly from the input buffer
/// without allocating new memory.
///
/// Benefits:
///
/// - fewer heap allocations
/// - improved performance
/// - reduced memory usage when processing large datasets
///
/// # Lifetime Parameter
///
/// The lifetime `'a` ties the event fields to the lifetime of the input buffer
/// used during deserialization.
///
/// This is a common pattern when building **high-performance streaming
/// pipelines in Rust**.
///
/// # Example JSON
///
/// Example event from GHArchive:
///
/// ```json
/// {
///   "id": "123456789",
///   "type": "PushEvent",
///   "repo": { "name": "rust-lang/rust" },
///   "actor": { "login": "octocat" },
///   "created_at": "2026-03-11T15:23:02Z"
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct GitHubEvent<'a> {
    /// Unique identifier for the event.
    #[serde(borrow)]
    pub id: &'a str,

    /// Repository information associated with the event.
    #[serde(borrow)]
    pub repo: Repo<'a>,

    /// The GitHub event type.
    ///
    /// Common examples:
    ///
    /// - `PushEvent`
    /// - `PullRequestEvent`
    /// - `WatchEvent`
    /// - `ForkEvent`
    ///
    /// The JSON field name is `type`, which is renamed here because
    /// `type` is a reserved keyword in Rust.
    #[serde(rename = "type")]
    pub kind: &'a str,

    /// Actor responsible for the event.
    #[serde(borrow)]
    pub actor: Actor<'a>,

    /// Timestamp when the event was created.
    ///
    /// Format:
    ///
    /// ```text
    /// 2026-03-11T15:23:02Z
    /// ```
    #[serde(borrow)]
    pub created_at: &'a str,
}

/// Repository metadata associated with a GitHub event.
///
/// Only the repository name is required for the purposes of
/// `octo-flow`, so the model contains a minimal subset of fields
/// from the GitHub event schema.
#[derive(Serialize, Deserialize)]
pub struct Repo<'a> {
    /// Repository name in `owner/repo` format.
    ///
    /// Example:
    ///
    /// ```text
    /// rust-lang/rust
    /// ```
    ///
    /// This field is optional because some GitHub events may not
    /// include repository metadata.
    #[serde(borrow)]
    pub name: Option<&'a str>,
}

/// Information about the GitHub user responsible for the event.
#[derive(Serialize, Deserialize)]
pub struct Actor<'a> {
    /// Username of the actor that triggered the event.
    ///
    /// Example:
    ///
    /// ```text
    /// octocat
    /// ```
    ///
    /// The field is optional because some events may omit actor data.
    #[serde(borrow)]
    pub login: Option<&'a str>,
}
