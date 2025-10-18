use serde::Deserialize;

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
