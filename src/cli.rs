use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Verbosity level
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
