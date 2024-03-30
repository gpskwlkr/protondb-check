use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Steam profile ID
    #[arg(short = 'p', long = "profile-id", required = false)]
    pub profile_id: Option<String>,

    ///
    #[arg(short = 'a', long = "app-id", required = false)]
    pub app_id: Option<u32>,
}

impl Args {
    pub fn new() -> Result<Self> {
        Ok(Args::parse())
    }

    pub fn validate_args(&self) -> Result<(), anyhow::Error> {
        if self.profile_id.is_none() && self.app_id.is_none() {
            return Err(anyhow!("Either --profile-id or --app-id must be provided"))
                .with_context(|| "validate_args");
        }

        Ok(())
    }
}
