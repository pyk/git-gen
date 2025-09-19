mod args;

use clap::Parser;

fn main() {
    let _args = args::Args::parse();
    println!("Hello, world!");
}
