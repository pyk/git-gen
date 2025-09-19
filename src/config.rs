use std::fs;

use serde::Deserialize;

use crate::error::Result;
use crate::{
    bail,
    error,
    git,
};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub provider: Provider,
    pub model: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Gemini,
    Grok,
    OpenAI,
}

pub fn load() -> Result<Config> {
    let repo_root = git::root()?;
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
