use clap::Parser;

mod args;
mod config;
mod context;
mod error;
mod git;
mod prompts;
mod providers;

use crate::{
    args::Args,
    error::Result,
    providers::Provider,
    providers::gemini::Gemini,
};

fn run() -> Result<()> {
    let args = Args::try_parse()?;
    let config = config::load()?;
    let context = context::create(&args, &config)?;
    let provider = match &config.provider {
        config::Provider::Gemini => Gemini::new(config.model.clone()),
        _ => bail!("provider not implemented yet"),
    };
    let prompt = prompts::get(&config.provider)?;
    let commits = provider.generate(&prompt, &context)?;
    println!("Configuration loaded successfully!");
    println!("{:#?}", config.provider);
    println!("{:#?}", config.model);
    println!("{:#?}", context);
    println!("{:#?}", commits);
    println!("Model: {}", provider.model());

    println!("\nSuggested commit messages:");
    for (i, commit) in commits.iter().enumerate() {
        println!("{}. {}", i + 1, commit);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
