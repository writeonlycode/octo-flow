use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_cli_filters_and_outputs_tab_separated_data() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup: Create a fake GHArchive file
    let mut file = NamedTempFile::new()?;

    writeln!(
        file,
        r#"{{"id":"1","type":"PushEvent","actor":{{"login":"octocat"}},"repo":{{"name":"repo1"}},"created_at":"2026-03-12"}}"#
    )?;
    writeln!(
        file,
        r#"{{"id":"2","type":"WatchEvent","actor":{{"login":"user2"}},"repo":{{"name":"repo2"}},"created_at":"2026-03-12"}}"#
    )?;

    // 2. Execution: Run the binary
    let mut cmd = Command::cargo_bin("octo-flow")?;
    cmd.arg("--input")
        .arg(file.path())
        .arg("--event")
        .arg("PushEvent");

    // 3. Assertion: Verify stdout and exit status
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "1\t2026-03-12\toctocat\trepo1\tPushEvent",
        ))
        .stdout(predicate::str::contains("WatchEvent").not()); // Ensure filtering worked

    Ok(())
}

#[test]
fn test_cli_fails_on_missing_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("octo-flow")?;
    cmd.arg("--input").arg("non_existent_file.json");

    // Verify it fails with our custom error message
    cmd.assert().failure().stderr(predicate::str::contains(
        "The octo-flow engine encountered a critical error during processing.",
    ));

    Ok(())
}
