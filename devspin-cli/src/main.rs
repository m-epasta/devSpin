//! DEVSPIN

use clap::Parser;
use devspin_cli::cli::Cli;

#[tokio::main]
async fn main() {
    println!("Hello, Devspin!");

    let cli = Cli::parse();
    #[allow(unused_must_use)]
    cli.execute().await;

}
