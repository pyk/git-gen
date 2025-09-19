use crate::error::Result;

pub mod gemini;

pub trait Provider {
    /// Generates commit messages based on the given prompt and context
    fn generate<C>(&self, prompt: &str, context: C) -> Result<Vec<String>>
    where
        C: envfmt::Context;
}
