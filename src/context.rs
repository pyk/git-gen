use crate::error::Result;
use crate::git;

#[derive(Debug)]
pub struct Context {
    pub git_diff: String,
    pub git_log: String,
}

pub fn create() -> Result<Context> {
    let git_diff = git::diff()?;
    let git_log = git::previous_commits()?;
    Ok(Context { git_diff, git_log })
}
