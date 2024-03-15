use std::path::Path;
use std::process::{Child, Command, Stdio};

use crate::error::lib::LibError;
use crate::error::process::ProcessError;
use crate::utils::fs::{copy_dir_all, write_bytes_to_temp_file};

const CHAIN_SIMULATOR_BYTES: &[u8] = include_bytes!("../../assets/chainsimulator");
const LIBWASMER_BYTES: &[u8] = include_bytes!("../../assets/libwasmer_darwin_amd64.dylib");

const CHAIN_SIMULATOR_NAME: &str = "chainsimulator";
const LIBWASMER_NAME: &str = "libwasmer_darwin_amd64.dylib";

const CONFIG_FOLDER: &str = "config";
const CONFIG_NAME: &str = "config.toml";

pub fn spawn_simulator_process(tempdir_path: &Path, config_content: &[u8]) -> Result<Child, LibError> {
    let chain_simulator_path = write_bytes_to_temp_file(
        CHAIN_SIMULATOR_NAME,
        tempdir_path,
        CHAIN_SIMULATOR_BYTES,
        true
    )?;

    write_bytes_to_temp_file(
        LIBWASMER_NAME,
        tempdir_path,
        LIBWASMER_BYTES,
        false
    )?;

    let Ok(()) = copy_dir_all("assets/config", tempdir_path.join("config")) else {
        return Err(ProcessError::CannotCopyAssets.into())
    };

    write_bytes_to_temp_file(
        CONFIG_NAME,
        tempdir_path.join(CONFIG_FOLDER).as_path(),
        config_content,
        false
    )?;

    Command::new(&chain_simulator_path)
        .current_dir(&tempdir_path)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| ProcessError::CannotSpawnProcess.into())
}