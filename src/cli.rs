use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
