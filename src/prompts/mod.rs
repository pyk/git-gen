use crate::bail;
use crate::config::Provider;
use crate::error::Result;

pub fn get(provider: &Provider) -> Result<String> {
    let prompt = match provider {
        Provider::Gemini => include_str!("./gemini.md"),
        _ => bail!("No prompt for this provider yet"),
    };

    Ok(prompt.to_string())
}
