use std::fs;

use crate::bail;
use crate::config::Config;
use crate::error;
use crate::error::Result;
use crate::git;

/// Represents the entire parsed content of the GITGEN.md manifest file.
#[derive(Debug)]
pub struct Manifest {
    pub config: Config,
    pub user_prompt: String,
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

    parse(&content).map_err(|e| {
        e.note(format!(
            "the error occurred in file '{}'",
            manifest_path.display()
        ))
    })
}

/// Parses the content of a GITGEN.md file.
pub fn parse(content: &str) -> Result<Manifest> {
    if !content.starts_with("---") {
        bail!("manifest must start with TOML frontmatter delimited by '---'");
    }
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        bail!("manifest frontmatter is not closed with '---'");
    }

    let frontmatter_str = parts[1];
    let prompt_str = parts[2];

    if frontmatter_str.trim().is_empty() {
        bail!(
            "TOML frontmatter cannot be empty",
            help: "you must specify at least the 'provider'"
        );
    }

    if prompt_str.trim().is_empty() {
        bail!("manifest is missing the prompt after the frontmatter");
    }

    let config: Config = toml::from_str(frontmatter_str).map_err(|e| {
        error!(
            "failed to parse TOML frontmatter in manifest",
            source: e,
            help: "please check for syntax errors or invalid values in your manifest config"
        )
    })?;

    Ok(Manifest {
        config,
        user_prompt: prompt_str.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_manifest() {
        let content = r#""#;
        let result = parse(content);
        let error = result.unwrap_err();
        assert!(error.message().contains(
            "manifest must start with TOML frontmatter delimited by '---'"
        ));
    }

    #[test]
    fn test_parse_empty_frontmatter() {
        let content = r#"---
---

some prompt
"#;
        let result = parse(content);
        let error = result.unwrap_err();
        assert!(error.message().contains("TOML frontmatter cannot be empty"));
    }

    #[test]
    fn test_parse_empty_prompt() {
        let content = r#"---
random
---
"#;
        let result = parse(content);
        let error = result.unwrap_err();
        assert!(
            error.message().contains(
                "manifest is missing the prompt after the frontmatter"
            )
        );
    }

    #[test]
    fn test_parse_invalid_config() {
        let content = r#"---
random
---
some prompt
"#;
        let result = parse(content);
        let error = result.unwrap_err();
        println!("{}", error.message());
        assert!(
            error
                .message()
                .contains("failed to parse TOML frontmatter in manifest")
        );
    }
}
