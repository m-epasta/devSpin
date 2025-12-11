use crate::prelude::*;
use std::path::PathBuf;

pub fn create_cfg_file(root: &str) -> Result<PathBuf, ProcessError> {
    // Create PathBuf for the root directory
    let mut file_path = PathBuf::from(root);

    // Append filename
    file_path.push("devspin.yml");

    // Create the file
    std::fs::File::create(&file_path).map_err(|e| ProcessError::CreateFileFailed {
        error_msg: e.to_string(),
        exit_status: 400,
    })?;

    Ok(file_path)
}
