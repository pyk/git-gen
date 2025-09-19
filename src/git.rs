use std::process::Stdio;
use std::{
    path::PathBuf,
    process::Command,
};

use crate::error::Result;
use crate::{
    bail,
    error,
};

/// Gets the root of the git repo via `git rev-parse --show-toplevel`
pub fn root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| {
            error!(
                "failed to execute git command",
                source: e,
                help: "ensure that 'git' is installed and in your system's PATH"
            )
        })?;

    if !output.status.success() {
        bail!(
            "not a git repository (or any of the parent directories)",
            help: "please run 'git commitgen' from within a git repository"
        );
    }

    let stdout = String::from_utf8(output.stdout).map_err(|e| {
        error!(
            "failed to parse 'git rev-parse --show-toplevel' command output",
            source: e,
            note: "the output from 'git rev-parse --show-toplevel' was not valid UTF-8"
        )
    })?;

    Ok(PathBuf::from(stdout.trim()))
}

/// Gets staged diff via `git diff --staged`
pub fn diff() -> Result<String> {
    let output = Command::new("git")
        .args(["diff", "--staged"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| {
            error!(
                "failed to execute git command",
                source: e,
                help: "ensure that 'git' is installed and in your system's PATH"
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git diff failed: {}", stderr);
    }

    let diff = String::from_utf8_lossy(&output.stdout).to_string();
    if diff.trim().is_empty() {
        bail!(
            "no staged changes found",
            help: "use `git add` to stage files before generating a commit message"
        );
    }

    Ok(diff)
}
