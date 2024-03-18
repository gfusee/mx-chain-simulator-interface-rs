use std::{fs, io};
use std::fs::{File, Permissions};
use std::io::Write;
use std::path::{Path, PathBuf};
use include_dir::Dir;
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
    set_permissions: bool
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

    if set_permissions {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let permissions_mode = 0o755;
            let Ok(()) = fs::set_permissions(&temp_file_path, Permissions::from_mode(permissions_mode)) else {
                let error = FsError::CannotSetPermissionsToFile {
                    file_path: temp_file_path.to_str().unwrap().to_string(),
                    permissions: Permissions::from_mode(permissions_mode),
                };

                return Err(error.into())
            };
        }
    }

    Ok(temp_file_path)
}

pub(crate) fn copy_dir_all(src: &Dir, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in src.files() {
        let file_dst = dst.as_ref().join(entry.path().file_name().unwrap());
        fs::write(file_dst, entry.contents())?;
    }

    for dir in src.dirs() {
        let next_dst = dst.as_ref().join(dir.path().file_name().unwrap());
        copy_dir_all(dir, next_dst)?;
    }

    Ok(())
}