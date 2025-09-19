use std::fs;
use std::{
    path::PathBuf,
    process::Command,
};

use serde::Deserialize;

use crate::error::Result;
use crate::{
    bail,
    error,
};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub provider: String,
    pub model: Option<String>,
}

/// Gets the root of the git repo via `git rev-parse --show-toplevel`
fn get_repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| {
            error!(
                "Failed to execute git command",
                source: e,
                help: "Ensure that 'git' is installed and in your system's PATH"
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        bail!(
            "Not a git repository (or any of the parent directories)",
            note: stderr,
            help: "Please run `git commitgen` from within a git repository"
        );
    }

    let stdout = String::from_utf8(output.stdout).map_err(|e| {
        error!(
            "Failed to parse git command output",
            source: e,
            note: "The output from 'git rev-parse --show-toplevel' was not valid UTF-8"
        )
    })?;

    Ok(PathBuf::from(stdout.trim()))
}

pub fn load() -> Result<Config> {
    let repo_root = get_repo_root()?;
    let config_path = repo_root.join("commitgen.toml");

    if !config_path.exists() {
        bail!(
            "Config file 'commitgen.toml' not found at repository root",
            help: format!(
                "Please create a 'commitgen.toml' file in the root of your git repository at '{}'",
                repo_root.display()
            )
        );
    }

    let content = fs::read_to_string(&config_path).map_err(|e| {
        error!(
            "Failed to read config file at '{}'", config_path.display(),
            source: e,
            help: "Please check the file's permissions."
        )
    })?;

    let config: Config = toml::from_str(&content).map_err(|e| {
        error!(
            "Failed to parse TOML in '{}'", config_path.display(),
            source: e,
            help: "Please check that the TOML syntax is valid."
        )
    })?;

    Ok(config)
}
