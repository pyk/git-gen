use std::time::Duration;
use std::{
    env,
    thread,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    bail,
    error,
};
use crate::{
    error::Result,
    providers::Provider,
};

pub struct Gemini {
    model: String,
    url: String,
}

impl Gemini {
    pub fn new(model: Option<String>) -> Self {
        let model = model.unwrap_or("gemini-2.5-flash-lite".to_string());
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            model
        );
        Self { model, url }
    }

    pub fn model(&self) -> String {
        self.model.clone()
    }
}

#[derive(Serialize)]
struct Request {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Debug, Deserialize)]
struct ContentResponse {
    parts: Vec<Part>,
}

impl Provider for Gemini {
    fn generate<C>(&self, template: &str, context: C) -> Result<Vec<String>>
    where
        C: envfmt::Context,
    {
        let api_key = env::var("GEMINI_API_KEY").map_err(|e| {
            error!(
                "failed to read GEMINI_API_KEY",
                source: e,
                help: "please make sure GEMINI_API_KEY is defined"
            )
        })?;

        let prompt = envfmt::format_with(template, &context).map_err(|e| {
            error!(
                "failed to generate prompt from template and context",
                source: e
            )
        })?;

        let payload = Request {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        // Try up to 5 times (1 initial + 5 retries)
        const MAX_RETRIES: u32 = 5;
        let mut attempts = 0;

        let mut response = loop {
            attempts += 1;

            let result = ureq::post(&self.url)
                .header("X-goog-api-key", &api_key)
                .send_json(&payload);

            match result {
                Ok(res) => {
                    break res;
                }
                // 404 is a permanent error, no point in retrying
                Err(ureq::Error::StatusCode(404)) => {
                    bail!(
                        "unknown model: {}", self.model,
                        help: "review your commitgen.toml and make sure its valid model name"
                    );
                }
                // A 5xx error is a server-side issue, so we can retry
                Err(ureq::Error::StatusCode(code))
                    if (500..=599).contains(&code) =>
                {
                    if attempts >= MAX_RETRIES {
                        bail!(
                            "Server error after {} attempts with status code: {}",
                            attempts,
                            code
                        );
                    }

                    // Calculate wait time with exponential backoff + jitter
                    let backoff_secs = 2u64.pow(attempts);
                    let jitter_ms = rand::random::<u16>() % 1000;
                    let wait_time = Duration::from_secs(backoff_secs)
                        + Duration::from_millis(jitter_ms as u64);

                    eprintln!(
                        "Server error ({}), retrying in {:?} (attempt {}/{})",
                        code, wait_time, attempts, MAX_RETRIES
                    );
                    thread::sleep(wait_time);
                }
                // Any other error is treated as permanent
                Err(err) => {
                    bail!(
                        "unexpected error when request model: {}", self.model,
                        source: err,
                        help: "review your commitgen.toml and make sure its valid model name"
                    );
                }
            }
        };

        let data =
            response.body_mut().read_json::<Response>().map_err(|err| {
                error!("failed to deserialize Gemini API response",
                    source: err
                )
            })?;

        let commits: Vec<String> = data
            .candidates
            .iter()
            .flat_map(|c| &c.content.parts)
            .flat_map(|p| p.text.split("\n---\n"))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(commits)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn unknown_model() {
        let model = "random".to_string();
        let gemini = Gemini::new(Some(model));
        let prompt = include_str!("../prompts/gemini.md");
        let mut context = HashMap::new();
        context.insert("DRAFT_MESSAGE", "test");
        context.insert("GIT_COMMITS", "test");
        context.insert("GIT_DIFF", "test");
        let result = gemini.generate(prompt, context);
        let error = result.unwrap_err();
        assert!(error.message().contains("unknown model"));
    }

    #[test]
    fn known_model() {
        let model = "gemini-2.5-flash-lite".to_string();
        let gemini = Gemini::new(Some(model));
        let prompt = include_str!("../prompts/gemini.md");
        let mut context = HashMap::new();
        context.insert("DRAFT_MESSAGE", "add more dependencies");
        context.insert("GIT_COMMITS", r#"
            3943e8b (HEAD -> main, origin/main) feat: add context and git modules
            7730132 chore: add commitgen
            9e5e081 feat: use enum for the provider
            68bcf27 fix: improves error message
            ce084d5 chore: add config loader
            e281295 chore: add error handling
            d07b2cb feat: build the man page
            d1f74f2 chore: ignore repomix files
            f2638c6 feat: add clap
            0d54d47 chore: add github ci
        "#,
        );
        context.insert("GIT_DIFF", r#"
            diff --git a/Cargo.toml b/Cargo.toml
            index 4583acd..80fb708 100644
            --- a/Cargo.toml
            +++ b/Cargo.toml
            @@ -12,10 +12,12 @@ categories = ["command-line-utilities"]

             [dependencies]
             clap = { version = "4.5", features = ["derive"] }  # Command-line args parser
            -serde = { version = "1", features = ["derive"] }
            -toml = "0.9"
            +envfmt = "0.1"  # Variable expands
            +serde = { version = "1", features = ["derive"] }  # Handle JSON and TOML
            +toml = "0.9"  # TOML parsing
            +ureq = { version = "3", features = ["json"] }  # Send request

             [build-dependencies]
             clap = { version = "4.5", features = ["derive"] }  # Command-line args parser
            -clap_mangen = { version = "0.2" }
            +clap_mangen = { version = "0.2" }  # Generate man page
        "#);
        let result = gemini.generate(prompt, context).unwrap();
        assert_eq!(result.len(), 5);
    }
}
