use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub provider: Provider,
    pub model: Option<String>,
    pub examples: Option<String>,
    #[serde(default = "default_tone_preset")]
    pub tone_preset: TonePreset,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Gemini,
    Grok,
    OpenAI,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TonePreset {
    Personal,
    Conventional,
    Formal,
}

fn default_tone_preset() -> TonePreset {
    TonePreset::Conventional
}
