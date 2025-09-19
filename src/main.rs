use clap::Parser;

mod args;
mod config;
mod context;
mod error;
mod git;

use args::Args;
use error::Result;

fn run() -> Result<()> {
    let _args = Args::try_parse()?;
    let config = config::load()?;
    let context = context::create()?;
    println!("Configuration loaded successfully!");
    println!("{:#?}", config.provider);
    println!("{:#?}", config.model);
    println!("{:#?}", context);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
