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

pub fn load() -> Result<Config> {
    let repo_root = get_repo_root()?;
    let config_path = repo_root.join("commitgen.toml");

    if !config_path.exists() {
        bail!(
            "config file 'commitgen.toml' not found at repository root",
            help: format!(
                "please create a 'commitgen.toml' file in the root of your git repository at '{}'",
                repo_root.display()
            )
        );
    }

    let content = fs::read_to_string(&config_path).map_err(|e| {
        error!(
            "failed to read config file at '{}'", config_path.display(),
            source: e,
            help: "please check the file's permissions"
        )
    })?;

    let config: Config = toml::from_str(&content).map_err(|e| {
        error!(
            "failed to parse and deserialize TOML in '{}'", config_path.display(),
            source: e,
            help: "please check check for syntax errors or invalid values"
        )
    })?;

    Ok(config)
}
