use crate::prelude::*;
use clap::{Parser, Subcommand};

#[derive(Args, Debug)]
pub struct ConfigArgs {
    /*
    Add fields as needed for the config command
    */
}

impl ConfigArgs {
    pub async fn execute(&self) -> Result<(), ProcessError> {
        println!("Configuration management coming soon!");
        Ok(())
    }
}