use std::fmt::{Display, Formatter};
use crate::error::lib::LibError;

#[derive(PartialEq, Debug, Clone)]
pub enum ProcessError {
    CannotSpawnProcess,
    CannotCopyAssets
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::CannotSpawnProcess => {
              write!(f, "Cannot spawn process")
            },
            ProcessError::CannotCopyAssets => {
                write!(f, "Cannot copy chain simulator assets")
            },
        }
    }
}

impl From<ProcessError> for LibError {
    fn from(value: ProcessError) -> Self {
        LibError::Process(value)
    }
}