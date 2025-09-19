use crate::error::Result;
use crate::git;

#[derive(Debug)]
pub struct Context {
    pub draft_commit: Option<String>,
    pub git_diff: String,
    pub git_commits: String,
}

impl envfmt::Context for &Context {
    fn get(&self, key: &str) -> Option<String> {
        match key {
            "DRAFT_COMMIT" => self.draft_commit.clone(),
            "GIT_DIFF" => Some(self.git_diff.clone()),
            "GIT_COMMITS" => Some(self.git_commits.clone()),
            _ => None,
        }
    }
}

pub fn create(draft_commit: Option<String>) -> Result<Context> {
    let git_diff = git::diff()?;
    let git_commits = git::previous_commits()?;
    Ok(Context {
        draft_commit,
        git_diff,
        git_commits,
    })
}
