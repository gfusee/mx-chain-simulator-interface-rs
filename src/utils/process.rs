use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use include_dir::{Dir, include_dir};

use crate::error::lib::LibError;
use crate::error::process::ProcessError;
use crate::utils::fs::{copy_dir_all, write_bytes_to_temp_file};

const CHAIN_SIMULATOR_NAME: &str = "chainsimulator";
const LIBWASMER_NAME: &str = "libwasmer_darwin_amd64.dylib";

const CONFIG_FOLDER: &str = "config";
const CONFIG_NAME: &str = "config.toml";

static ASSETS: Dir = include_dir!("assets");

pub fn prepare_temp_dir_for_simulator(tempdir_path: &Path) -> Result<PathBuf, LibError> {
    let chain_simulator_path = write_bytes_to_temp_file(
        CHAIN_SIMULATOR_NAME,
        tempdir_path,
        ASSETS.get_file(CHAIN_SIMULATOR_NAME).unwrap().contents(),
        true
    )?;

    write_bytes_to_temp_file(
        LIBWASMER_NAME,
        tempdir_path,
        ASSETS.get_file(LIBWASMER_NAME).unwrap().contents(),
        false
    )?;

    let Ok(()) = copy_dir_all(&ASSETS.get_dir("config").unwrap(), tempdir_path.join("config")) else {
        return Err(ProcessError::CannotCopyAssets.into())
    };

    Ok(chain_simulator_path)
}

pub fn spawn_simulator_process<Args, S>(tempdir_path: &Path, config_content: &[u8], args: Args) -> Result<Child, LibError>
where
    Args: IntoIterator<Item = S>,
    S: AsRef<OsStr>
{
    let chain_simulator_path = tempdir_path.join(CHAIN_SIMULATOR_NAME);

    write_bytes_to_temp_file(
        CONFIG_NAME,
        tempdir_path.join(CONFIG_FOLDER).as_path(),
        config_content,
        false
    )?;

    Command::new(&chain_simulator_path)
        .current_dir(&tempdir_path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| ProcessError::CannotSpawnProcess.into())
}