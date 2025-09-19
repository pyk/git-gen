use clap::{
    ColorChoice,
    Parser,
};

#[derive(Parser, Debug)]
#[command(
    bin_name = "git commitgen",
    color = ColorChoice::Auto,
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = env!("CARGO_PKG_DESCRIPTION"),
    after_help = "Run `git commitgen help` for more detailed information."
)]
struct Args {
    /// Changes intention
    #[arg(short, long)]
    intent: Option<String>,
}

fn main() {
    let _args = Args::parse();

    println!("Hello, world!");
}
