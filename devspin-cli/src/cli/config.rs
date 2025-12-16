use crate::prelude::*;
// use clap::{Parser, Subcommand};

#[derive(clap::Args, Debug)]
pub struct ConfigArgs {
    /*
    Add fields as needed for the config command
    */
}

#[allow(dead_code)]
impl ConfigArgs {
    pub async fn execute(&self) -> Result<(), ProcessError> {
        println!("Configuration management coming soon!");
        Ok(())
    }
}
