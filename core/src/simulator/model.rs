use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use nix::Error;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use reqwest::Client;
use tempfile::TempDir;
use tokio::sync::Mutex;

use crate::error::lib::LibError;
use crate::error::requests::generate_blocks::GenerateBlocksError;
use crate::error::requests::initial_wallets::InitialWalletsError;
use crate::error::requests::set_address_keys::SetAddressKeysError;
use crate::error::requests::set_state::SetStateError;
use crate::error::simulator::SimulatorError;
use crate::simulator::config::SimulatorConfig;
use crate::simulator::process::SimulatorProcess;
use crate::simulator::requests::generate_blocks::GenerateBlocksResponse;
use crate::simulator::requests::initial_wallets::{InitialWallets, InitialWalletsResponse};
use crate::simulator::requests::set_address_keys::SetAddressKeysResponse;
use crate::simulator::requests::set_state::SetStateAddress;
use crate::SimulatorOptions;
use crate::utils::fs::get_temp_dir;
use crate::utils::process::{prepare_temp_dir_for_simulator, spawn_simulator_process};

pub struct Simulator {
    process_id_and_options: Arc<Mutex<Option<(u32, SimulatorOptions)>>>,
    tempdir: Arc<TempDir>,
}

impl Drop for Simulator {
    fn drop(&mut self) {
        let process_id_and_options_mutex = self.process_id_and_options.clone();
        tokio::spawn(async move {
            let _ = kill_simulator_process(process_id_and_options_mutex).await; // We ignore the result bc in the drop method we want to kill the process if it exists.
        });
    }
}

impl Simulator {
    pub fn new() -> Result<Simulator, LibError> {
        let tempdir = get_temp_dir()?;

        prepare_temp_dir_for_simulator(tempdir.path())?;

        let simulator = Simulator {
            process_id_and_options: Arc::new(Mutex::new(None)),
            tempdir: tempdir.into(),
        };

        Ok(simulator)
    }

    pub async fn start(&self, options: SimulatorOptions) -> Result<SimulatorProcess, LibError> {
        let _ = self.kill().await; // We ignore the result bc in the start method we want to kill the old process if it exists.

        let cli_args: Vec<String> = options.to_cli_args();
        let config = SimulatorConfig::from(options);
        let child = spawn_simulator_process(self.tempdir.path(), &config.get_toml_content()?, &cli_args)?;
        wait_to_be_ready(options.server_port, Duration::from_secs(10)).await?;

        let mut opt_process_and_options = self.process_id_and_options.lock().await;
        *opt_process_and_options = Some((child.id(), options));
        drop(opt_process_and_options);

        self.generate_epochs(1).await?;

        if let Some(block_autogenerate_duration) = options.block_autogenerate_duration {
            let process_id_and_options_mutex = self.process_id_and_options.clone();
            tokio::spawn(async move {
                autogenerate_blocks(block_autogenerate_duration, process_id_and_options_mutex, options.server_port).await
            });
        }

        Ok(SimulatorProcess(child))
    }

    pub async fn generate_blocks(&self, num_blocks: u64) -> Result<(), LibError> {
        let (_, options) = self.get_process_id_and_options().await?;

        generate_blocks(options.server_port, num_blocks).await
    }

    pub async fn get_initial_wallets(&self) -> Result<InitialWallets, LibError> {
        let (_, options) = self.get_process_id_and_options().await?;

        let url = format!("http://localhost:{}/simulator/initial-wallets", options.server_port);

        let Ok(response) = Client::new()
            .get(&url)
            .send()
            .await
            else {
                return Err(InitialWalletsError::CannotGetTextFromTheResponse { url }.into());
            };

        if !response.status().is_success() {
            return Err(InitialWalletsError::ResponseStatusIsNotSuccessful { url, status: response.status().as_u16() }.into());
        }

        let Ok(text) = response.text().await else {
            return Err(InitialWalletsError::CannotGetTextFromTheResponse { url }.into());
        };

        let Ok(result) = serde_json::from_str::<InitialWalletsResponse>(&text) else {
            return Err(InitialWalletsError::FailedToParseTheResponse { url, response: text }.into());
        };

        if result.code != "successful" {
            return Err(InitialWalletsError::ResponseCodeIsNotSuccessful { url, code: result.code }.into());
        }

        let Some(data) = result.data else {
            return Err(InitialWalletsError::ResponseCodeIsNotSuccessful { url, code: result.code }.into());
        };

        Ok(data)
    }

    pub async fn generate_epochs(&self, num_epochs: u64) -> Result<(), LibError> {
        let (_, options) = self.get_process_id_and_options().await?;

        let blocks_to_generate = (options.rounds_per_epoch + 1) * num_epochs;

        self.generate_blocks(blocks_to_generate).await
    }

    pub async fn set_address_keys(&self, address: &str, keys: &HashMap<String, String>) -> Result<(), LibError> {
        let (_, options) = self.get_process_id_and_options().await?;

        let url = format!("http://localhost:{}/simulator/address/{}/set-state", options.server_port, address);

        let Ok(body) = serde_json::to_string(keys) else {
            return Err(SetAddressKeysError::CannotConvertKeysAsJSON { url: url, keys: keys.clone() }.into())
        };

        let Ok(response) = Client::new()
            .post(&url)
            .body(body)
            .send()
            .await
            else {
                return Err(SetAddressKeysError::CannotGetTextFromTheResponse { url }.into());
            };

        if !response.status().is_success() {
            return Err(SetAddressKeysError::ResponseStatusIsNotSuccessful { url, status: response.status().as_u16() }.into());
        }

        let Ok(text) = response.text().await else {
            return Err(SetAddressKeysError::CannotGetTextFromTheResponse { url }.into());
        };

        let Ok(result) = serde_json::from_str::<SetAddressKeysResponse>(&text) else {
            return Err(SetAddressKeysError::FailedToParseTheResponse { url, response: text }.into());
        };

        if result.code != "successful" {
            return Err(SetAddressKeysError::ResponseCodeIsNotSuccessful { url, code: result.code }.into());
        }

        Ok(())
    }

    pub async fn set_state(&self, state: &[SetStateAddress]) -> Result<(), LibError> {
        let (_, options) = self.get_process_id_and_options().await?;

        let url = format!("http://localhost:{}/simulator/set-state", options.server_port);

        let Ok(body) = serde_json::to_string(state) else {
            return Err(SetStateError::CannotConvertStateAsJSON { url, state: state.to_vec() }.into())
        };

        let Ok(response) = Client::new()
            .post(&url)
            .body(body)
            .send()
            .await
            else {
                return Err(SetStateError::CannotGetTextFromTheResponse { url }.into());
            };

        if !response.status().is_success() {
            return Err(SetStateError::ResponseStatusIsNotSuccessful { url, status: response.status().as_u16() }.into());
        }

        let Ok(text) = response.text().await else {
            return Err(SetStateError::CannotGetTextFromTheResponse { url }.into());
        };

        let Ok(result) = serde_json::from_str::<SetAddressKeysResponse>(&text) else {
            return Err(SetStateError::FailedToParseTheResponse { url, response: text }.into());
        };

        if result.code != "successful" {
            return Err(SetStateError::ResponseCodeIsNotSuccessful { url, code: result.code }.into());
        }

        Ok(())
    }

    pub async fn autogenerate_blocks(&self, each: Duration) -> Result<(), LibError> {
        let (_, options) = self.get_process_id_and_options().await?;
        autogenerate_blocks(each, self.process_id_and_options.clone(), options.server_port).await?;

        Ok(())
    }

    async fn get_process_id_and_options(&self) -> Result<(u32, SimulatorOptions), LibError> {
        let opt_process_and_options = self.process_id_and_options.lock().await;

        let Some((process_id, options)) = opt_process_and_options.as_ref() else {
            return Err(SimulatorError::ProcessNotStarted.into());
        };

        if !is_process_running(*process_id) {
            return Err(SimulatorError::ProcessAlreadyFinished.into());
        }

        Ok((*process_id, *options))
    }

    async fn kill(&self) -> Result<(), LibError> {
        kill_simulator_process(self.process_id_and_options.clone()).await
    }
}

async fn wait_to_be_ready(server_port: u16, timeout: Duration) -> Result<(), LibError> {
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let url = format!("http://localhost:{}/about", server_port);

    loop {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        if current_time > start_time + timeout {
            return Err(SimulatorError::TimedOutWhileWaitingToBeReady.into())
        }

        let response = reqwest::get(&url).await;

        if response.is_ok() {
            let response = response.unwrap();
            if response.status().is_success() {
                break
            }
        }

        tokio::time::sleep(Duration::from_millis(10)).await
    }

    Ok(())
}

async fn generate_blocks(server_port: u16, num_blocks: u64) -> Result<(), LibError> {
    let url = format!("http://localhost:{}/simulator/generate-blocks/{}", server_port, num_blocks);

    let Ok(response) = Client::new()
        .post(&url)
        .send()
        .await
        else {
            return Err(GenerateBlocksError::CannotGetTextFromTheResponse { url }.into());
        };

    if !response.status().is_success() {
        return Err(GenerateBlocksError::ResponseStatusIsNotSuccessful { url, status: response.status().as_u16() }.into());
    }

    let Ok(text) = response.text().await else {
        return Err(GenerateBlocksError::CannotGetTextFromTheResponse { url }.into());
    };

    let Ok(result) = serde_json::from_str::<GenerateBlocksResponse>(&text) else {
        return Err(GenerateBlocksError::FailedToParseTheResponse { url, response: text }.into());
    };

    if result.code != "successful" {
        return Err(GenerateBlocksError::ResponseCodeIsNotSuccessful { url, code: result.code }.into());
    }

    Ok(())
}

async fn autogenerate_blocks(each: Duration, process_and_options_mutex: Arc<Mutex<Option<(u32, SimulatorOptions)>>>, server_port: u16) -> Result<(), LibError> {
    loop {
        let is_process_active = {
            let process_and_options = process_and_options_mutex.lock().await;

            process_and_options.is_some()
        };

        if !is_process_active {
            break
        }

        generate_blocks(server_port, 1).await?;

        tokio::time::sleep(each).await;
    }

    Ok(())
}

async fn kill_simulator_process(process_and_options_mutex: Arc<Mutex<Option<(u32, SimulatorOptions)>>>) -> Result<(), LibError> {
    let (process_id, _) = get_simulator_process_id_and_options(process_and_options_mutex.clone()).await?;

    let mut opt_process_id_and_options = process_and_options_mutex.lock().await;
    let _ = kill(Pid::from_raw(process_id as i32), Signal::SIGKILL);
    *opt_process_id_and_options = None;

    Ok(())
}

async fn get_simulator_process_id_and_options(process_and_options_mutex: Arc<Mutex<Option<(u32, SimulatorOptions)>>>) -> Result<(u32, SimulatorOptions), LibError> {
    let opt_process_and_options = process_and_options_mutex.lock().await;

    let Some((process_id, options)) = opt_process_and_options.as_ref() else {
        return Err(SimulatorError::ProcessNotStarted.into());
    };

    if !is_process_running(*process_id) {
        return Err(SimulatorError::ProcessAlreadyFinished.into());
    }

    Ok((*process_id, *options))
}

fn is_process_running(pid: u32) -> bool {
    match kill(Pid::from_raw(pid as i32), None) {
        Ok(_) => true,
        Err(Error::ESRCH) => false, // No such process
        Err(_) => true, // The process exists but we don't have permission to send the signal
    }
}