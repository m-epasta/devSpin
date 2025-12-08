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
}

impl Cli {
    pub async fn execute(self) -> Result<(), anyhow::Error> {
        match self.commands {
            Commands::TestCmd(args) => args.execute().await,
            Commands::Config => {
                println!("Configuration management coming soon!");
                Ok(())
            }
        }
    }
}

mod test_cmd;
