use std::{fs, io};
use std::ffi::OsStr;
use std::fs::Permissions;
use std::path::Path;
use std::process::{Child, Command, Stdio};

use crate::ASSETS_PATH;
use crate::error::fs::FsError;
use crate::error::lib::LibError;
use crate::error::process::ProcessError;
use crate::utils::fs::write_bytes_to_temp_file;

const CHAIN_SIMULATOR_NAME: &str = "chainsimulator";

const CONFIG_FOLDER: &str = "config";
const CONFIG_NAME: &str = "config.toml";

pub fn prepare_temp_dir_for_simulator(tempdir_path: &Path) -> Result<(), LibError> {
    copy_dir_recursive(&Path::new(ASSETS_PATH), tempdir_path)
        .map_err(|_| FsError::CannotCopyAssets.into())
}

pub fn spawn_simulator_process<Args, S>(tempdir_path: &Path, config_content: &[u8], args: Args) -> Result<Child, LibError>
where
    Args: IntoIterator<Item = S>,
    S: AsRef<OsStr>
{
    let chain_simulator_path = tempdir_path.join(CHAIN_SIMULATOR_NAME);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let permissions_mode = 0o755;
        let Ok(()) = fs::set_permissions(&chain_simulator_path, Permissions::from_mode(permissions_mode)) else {
            let error = FsError::CannotSetPermissionsToFile {
                file_path: chain_simulator_path.to_str().unwrap().to_string(),
                permissions: Permissions::from_mode(permissions_mode),
            };

            return Err(error.into())
        };
    }

    write_bytes_to_temp_file(
        CONFIG_NAME,
        tempdir_path.join(CONFIG_FOLDER).as_path(),
        config_content,
    )?;

    Command::new(&chain_simulator_path)
        .current_dir(&tempdir_path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| ProcessError::CannotSpawnProcess.into())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_file() {
            fs::copy(&src_path, &dst_path)?;
        } else if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        }
    }

    Ok(())
}