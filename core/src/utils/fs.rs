use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use tempfile::{tempdir, TempDir};

use crate::error::fs::FsError;
use crate::error::lib::LibError;

pub fn get_temp_dir() -> Result<TempDir, LibError> {
    tempdir()
        .map_err(|_| FsError::CannotGetTempDir.into())
}

pub(crate) fn write_bytes_to_temp_file(
    name: &str,
    dir: &Path,
    bytes: &[u8],
) -> Result<PathBuf, LibError> {
    let temp_file_path = dir.join(name);
    let Ok(mut file) = File::create(&temp_file_path) else {
        return Err(FsError::CannotCreateFile { file_path: temp_file_path.to_str().unwrap().to_string() }.into())
    };
    let Ok(()) = file.write_all(bytes) else {
        let error = FsError::CannotWriteBytesToFile {
            file_path: temp_file_path.to_str().unwrap().to_string(),
            bytes: bytes.to_vec()
        };

        return Err(error.into())
    };

    drop(file);

    Ok(temp_file_path)
}