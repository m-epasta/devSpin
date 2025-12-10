use crate::utils::{devspin_finder::find_devspin_yml_parallel, root_finder::get_root_no_param};

use crate::prelude::*;
use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    // Add fields as needed for the init command
}

impl InitArgs {
    pub async fn execute(&self) -> Result<(), ProcessError> {
        let root = get_root_no_param().map_err(|e| ProcessError::RootNotFound {
            error_msg: format!("Could not find project root: {}", e),
            exit_status: 404,
        })?;
        // check if devspin is found
        find_devspin_yml_parallel(root).map_err(|e| ProcessError::DevspinYmlNotFound {
            error_msg: e,
            exit_status: (404),
        })?;

        Ok(())
    }
}
