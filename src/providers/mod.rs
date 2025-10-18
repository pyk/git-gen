use crate::error::Result;

pub mod gemini;

pub trait Provider {
    /// Generates commit messages based on the given prompt and context
    fn generate(&self, prompt: &str) -> Result<Vec<String>>;
}
