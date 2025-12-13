use crate::prelude::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "devspin")]
#[command(about = "Development environment manager")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    TestCmd(test_cmd::TestCmd),
    Config,
    Init(init::InitArgs),
}

impl Cli {
    pub async fn execute(self) -> Result<(), ProcessError> {
        match self.commands {
            Commands::TestCmd(args) => args.execute().await,
            Commands::Config => {
                println!("Configuration management coming soon!");
                Ok(())
            }
            Commands::Init(args) => args.execute().await,
        }
    }
}

mod init;
mod test_cmd;
mod config;