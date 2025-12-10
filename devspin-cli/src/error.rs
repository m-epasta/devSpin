use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Failed to find devspin.yml (error: {error_msg}  status: {exit_status})")]
    DevspinYmlNotFound { error_msg: String, exit_status: u16 },

    #[error("Failed to find root (error: {error_msg}  status: {exit_status})")]
    RootNotFound { error_msg: String, exit_status: u16 },
}
