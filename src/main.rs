mod args;
mod error;

use args::Args;
use error::Result;

use clap::Parser;

fn run() -> Result<()> {
    let _args = Args::try_parse()?;
    println!("Hello, world!");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
