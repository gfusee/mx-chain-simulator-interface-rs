use std::fmt::{Display, Formatter};
use crate::error::lib::LibError;

#[derive(PartialEq, Debug, Clone)]
pub enum ProcessError {
    CannotSpawnProcess,
    CannotCopyAssets,
    UnsupportedOSAndArch { os: String, arch: String }
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
            ProcessError::UnsupportedOSAndArch { os, arch } => {
                write!(f, "Unsupported OS and arch: {os} {arch}. Supported OS and arch: linux/amd64, darwin/amd64")
            },
        }
    }
}

impl From<ProcessError> for LibError {
    fn from(value: ProcessError) -> Self {
        LibError::Process(value)
    }
}