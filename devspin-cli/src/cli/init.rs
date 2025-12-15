use crate::utils::{
    create_devspin_file::create_cfg_file, devspin_finder::find_devspin_yml_parallel,
    root_finder::{get_root_no_param}
};

use crate::prelude::*;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(short, long)]
    root: Option<String>,
}

impl InitArgs {
    pub async fn execute(&self) -> Result<(), ProcessError> {
        let root = if let Some(root_path) = &self.root {
            let root_buf = PathBuf::from(root_path);
            if !root_buf.exists() {
                return Err(ProcessError::RootNotFound {
                    error_msg: format!("Specified root directory does not exist: {}", root_path),
                    exit_status: 404,
                });
            }
            if !root_buf.is_dir() {
                return Err(ProcessError::RootNotFound {
                    error_msg: format!("Specified root path is not a directory: {}", root_path),
                    exit_status: 404,
                });
            }
            root_buf
        } else {
            get_root_no_param()
                .map_err(|e| ProcessError::RootNotFound {
                    error_msg: format!("Could not find project root: {}", e),
                    exit_status: 404,
                })?
        };

        // Check if devspin.yml already exists (don't overwrite)
        if find_devspin_yml_parallel(&root).is_ok() {
            return Err(ProcessError::CreateFileFailed {
                error_msg: "devspin.yml already exists in this directory".to_string(),
                exit_status: 409,
            });
        }

        let root_str = root
            .to_str()
            .ok_or_else(|| ProcessError::CreateFileFailed {
                error_msg: "Invalid UTF-8 in path".to_string(),
                exit_status: 400,
            })?;

        create_cfg_file(root_str)
            .map_err(|e| ProcessError::CreateFileFailed {
                error_msg: e.to_string(),
                exit_status: 400,
            })?;

        Ok(())
    }
}
