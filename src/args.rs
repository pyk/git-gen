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
    after_help = "Run 'git commitgen help' for more detailed information."
)]
pub struct Args {
    /// Draft commit
    #[arg(short, long)]
    pub message: Option<String>,
}
