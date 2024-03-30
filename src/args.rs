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

    /// Steam Application ID (obtainable on Store page)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_args_no_args() {
        let args = Args {
            profile_id: None,
            app_id: None,
        };

        let result = args.validate_args();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "validate_args");
    }
}
