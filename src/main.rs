use clap::Parser;

mod args;
mod config;
mod error;

use args::Args;
use error::Result;

fn run() -> Result<()> {
    let _args = Args::try_parse()?;
    let config = config::load()?;
    println!("Configuration loaded successfully!");
    println!("{:#?}", config.provider);
    println!("{:#?}", config.model);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
