use std::fmt::{Display, Formatter};
use std::fs::Permissions;
use crate::error::lib::LibError;

#[derive(PartialEq, Debug, Clone)]
pub enum FsError {
    CannotGetTempDir,
    CannotCopyAssets,
    CannotCreateFile { file_path: String },
    CannotWriteBytesToFile { file_path: String, bytes: Vec<u8> },
    CannotSetPermissionsToFile { file_path: String, permissions: Permissions }
}

impl Display for FsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FsError::CannotGetTempDir => {
                write!(f, "Cannot get a temporary directory")
            },
            FsError::CannotCopyAssets => {
                write!(f, "Cannot copy assets")
            },
            FsError::CannotCreateFile { file_path } => {
              write!(f, "Cannot create a file at the specified path: {file_path}")
            },
            FsError::CannotWriteBytesToFile { file_path, bytes } => {
                write!(f, "Cannot write bytes of length {} in {file_path}", bytes.len())
            },
            FsError::CannotSetPermissionsToFile { file_path, permissions } => {
                write!(f, "Cannot set permissions {:?} to file {file_path}", permissions)
            },
        }
    }
}

impl From<FsError> for LibError {
    fn from(value: FsError) -> Self {
        LibError::Fs(value)
    }
}