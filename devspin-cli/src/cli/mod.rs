use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "devspin")]
#[command(about = "Developpement environnement manager")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    TestCmd(test_cmd::TestCmd),
}

impl Cli {
    pub async fn execute(self) -> Result<(), anyhow::Error> {
        match self.commands {
            Commands::TestCmd(args) => args.execute().await,
        }
    }
}

mod test_cmd;