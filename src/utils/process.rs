use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use include_dir::{Dir, include_dir};

use crate::error::lib::LibError;
use crate::error::process::ProcessError;
use crate::utils::fs::{copy_dir_all, write_bytes_to_temp_file};

const CHAIN_SIMULATOR_NAME: &str = "chainsimulator";
const CHAIN_SIMULATOR_DARWIN_AMD64_NAME: &str = "chainsimulator_darwin_amd64";
const CHAIN_SIMULATOR_LINUX_AMD64_NAME: &str = "chainsimulator_linux_amd64";
const LIBWASMER_LINUX_AMD64_NAME: &str = "libwasmer_linux_amd64.do";
const LIBWASMER_DARWIN_AMD64_NAME: &str = "libwasmer_darwin_amd64.dylib";

const CONFIG_FOLDER: &str = "config";
const CONFIG_NAME: &str = "config.toml";

static ASSETS: Dir = include_dir!("assets");

pub fn prepare_temp_dir_for_simulator(tempdir_path: &Path) -> Result<PathBuf, LibError> {
    let chain_simulator_asset = if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        ASSETS.get_file(CHAIN_SIMULATOR_LINUX_AMD64_NAME)
            .unwrap()
    } else if cfg!(all(target_os = "macos", any(target_arch = "x86_64", target_arch = "aarch64"))) {
        ASSETS.get_file(CHAIN_SIMULATOR_DARWIN_AMD64_NAME)
            .unwrap()
    } else {
        return Err(ProcessError::UnsupportedOSAndArch {
            os: env::consts::OS.to_string(),
            arch: env::consts::ARCH.to_string(),
        }.into());
    };

    let chain_simulator_path = write_bytes_to_temp_file(
        CHAIN_SIMULATOR_NAME,
        tempdir_path,
        chain_simulator_asset.contents(),
        true
    )?;

    let libwasmer_name = if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        LIBWASMER_LINUX_AMD64_NAME
    } else if cfg!(all(target_os = "macos", any(target_arch = "x86_64", target_arch = "aarch64"))) {
        LIBWASMER_DARWIN_AMD64_NAME
    } else {
        return Err(ProcessError::UnsupportedOSAndArch {
            os: env::consts::OS.to_string(),
            arch: env::consts::ARCH.to_string(),
        }.into());
    };

    write_bytes_to_temp_file(
        libwasmer_name,
        tempdir_path,
        ASSETS.get_file(libwasmer_name).unwrap().contents(),
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