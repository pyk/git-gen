use crate::args::Args;
use crate::config::Config;
use crate::error::Result;
use crate::git;

#[derive(Debug)]
pub struct Context {
    pub tone_preset: Option<String>,
    pub draft_commit: Option<String>,
    pub examples: Option<String>,
    pub git_diff: String,
    pub git_commits: String,
}

impl envfmt::Context for &Context {
    fn get(&self, key: &str) -> Option<String> {
        match key {
            "TONE_PRESET" => self.tone_preset.clone(),
            "DRAFT_COMMIT" => self.draft_commit.clone(),
            "EXAMPLES" => self.examples.clone(),
            "GIT_DIFF" => Some(self.git_diff.clone()),
            "GIT_COMMITS" => Some(self.git_commits.clone()),
            _ => None,
        }
    }
}

pub fn create(args: &Args, config: &Config) -> Result<Context> {
    let tone_preset = Some(format!("{:?}", config.tone_preset).to_lowercase());
    let draft_commit = args.message.clone();
    let examples = config.examples.clone();
    let git_diff = git::diff()?;
    let git_commits = git::previous_commits()?;
    Ok(Context {
        tone_preset,
        draft_commit,
        examples,
        git_diff,
        git_commits,
    })
}
