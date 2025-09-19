use std::collections::HashMap;

use crate::error::Result;
use crate::git;

#[derive(Debug)]
pub struct Context {
    pub diff: String,
}

impl From<Context> for HashMap<&str, String> {
    // HashMap to expands variable in the prompt file
    fn from(context: Context) -> Self {
        let mut map = HashMap::new();
        map.insert("diff", context.diff);
        map
    }
}

pub fn create() -> Result<Context> {
    let diff = git::diff()?;
    Ok(Context { diff })
}
