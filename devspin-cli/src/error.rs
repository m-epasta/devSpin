use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Failed to find devspin.yml (error: {error_msg}  status: {exit_status})")]
    DevspinYmlNotFound { error_msg: String, exit_status: u16 },

    #[error("Failed to find root (error: {error_msg}  status: {exit_status})")]
    RootNotFound { error_msg: String, exit_status: u16 },

    #[error("Failed to create file (error: {error_msg}  status: {exit_status})")]
    CreateFileFailed { error_msg: String, exit_status: u16 },
}

#[derive(Error, Debug)]
pub enum DevSpinError {
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Process error: {0}")]
    Process(#[from] ProcessError),
}
