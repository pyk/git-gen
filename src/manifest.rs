use std::fs;

use crate::bail;
use crate::config::Config;
use crate::error;
use crate::error::Result;
use crate::git;

/// Represents the entire parsed content of the GITGEN.md manifest file
#[derive(Debug)]
pub struct Manifest {
    pub config: Config,
    pub prompt: String,
}

/// Loads and parses the GITGEN.md manifest from the repository root.
pub fn load() -> Result<Manifest> {
    let repo_root = git::root()?;
    let manifest_path = repo_root.join("GITGEN.md");

    if !manifest_path.exists() {
        bail!(
            "manifest file 'GITGEN.md' not found at repository root",
            help: format!(
                "please create a 'GITGEN.md' file in the root of your git repository at '{}'",
                repo_root.display()
            )
        );
    }

    let content = fs::read_to_string(&manifest_path).map_err(|e| {
        error!(
            "failed to read manifest file at '{}'", manifest_path.display(),
            source: e,
             help: "please check file's permissions"
        )
    })?;

    if !content.starts_with("---") {
        bail!("'GITGEN.md' must start with TOML frontmatter delimited by ---");
    }
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        bail!("'GITGEN.md' frontmatter is not closed with '---'");
    }

    let frontmatter_str = parts[1];
    let prompt_str = parts[2];

    if prompt_str.trim().is_empty() {
        bail!("'GITGEN.md' is missing the prompt after the frontmatter");
    }

    let config: Config = toml::from_str(frontmatter_str).map_err(|e| {
        error!(
            "failed to parse TOML frontmatter in 'GITGEN.md' manifest",
            source: e,
            help: "please check check for syntax errors or invalid values"
        )
    })?;

    Ok(Manifest {
        config,
        prompt: prompt_str.to_string(),
    })
}
