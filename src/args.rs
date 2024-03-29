use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Steam profile ID
    #[arg(short = 'p', long = "profile-id", required = true)] 
    pub profile_id: String,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}
