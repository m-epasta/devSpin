//! MUST BE USED ONLY ON DEBUG; FORBIDDEN IN PRODUCTION

use clap::Args;

use crate::error::ProcessError;

#[derive(Debug, Args, Clone)]
pub struct TestCmd {
    /// Print DEVSPIN in ASCII art
    #[arg(short, long)]
    w_msg: bool,
}

impl TestCmd {
    pub async fn execute(&self) -> Result<(), ProcessError> {
        use colored::*;
        if self.w_msg {
            println!(
                "{}",
                r#"
    ██████╗ ███████╗██╗   ██╗███████╗██████╗ ██╗███╗   ██╗
    ██╔══██╗██╔════╝██║   ██║██╔════╝██╔══██╗██║████╗  ██║
    ██║  ██║█████╗  ██║   ██║███████╗██████╔╝██║██╔██╗ ██║
    ██║  ██║██╔══╝  ╚██╗ ██╔╝╚════██║██╔═══╝ ██║██║╚██╗██║
    ██████╔╝███████╗ ╚████╔╝ ███████║██║     ██║██║ ╚████║
    ╚═════╝ ╚══════╝  ╚═══╝  ╚══════╝╚═╝     ╚═╝╚═╝  ╚═══╝
    "#
                .bright_cyan()
                .bold()
            );
            println!("{}", "────────────────────────────────────────".cyan());
        }
        Ok(())
    }
}
