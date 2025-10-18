use clap::Parser;

mod args;
mod config;
mod context;
mod error;
mod git;
mod manifest;
mod prompt;
mod providers;

use crate::{
    args::Args,
    error::Result,
    providers::Provider,
    providers::gemini::Gemini,
};

fn run() -> Result<()> {
    let args = Args::try_parse()?;
    let manifest = manifest::load()?;
    let context = context::create()?;
    let final_prompt = prompt::create(
        args.message, //
        &manifest.user_prompt,
        &context,
    );

    let config = &manifest.config;
    let provider = match &config.provider {
        config::Provider::Gemini => Gemini::new(config.model.clone()),
        _ => bail!("provider not implemented yet"),
    };
    let commits = provider.generate(&final_prompt)?;

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
