use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Failed to find devspin.yml (status: {exit_status})")]
    DevspinYmlNotFound { exit_status: u16 },
}
