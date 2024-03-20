use std::sync::Arc;

use nix::Error;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use reqwest::Client;
use tempfile::TempDir;
use tokio::sync::Mutex;

use crate::error::lib::LibError;
use crate::error::requests::generate_blocks::GenerateBlocksError;
use crate::error::simulator::SimulatorError;
use crate::simulator::config::SimulatorConfig;
use crate::simulator::process::SimulatorProcess;
use crate::simulator::requests::generate_blocks::GenerateBlocksResponse;
use crate::SimulatorOptions;
use crate::utils::fs::get_temp_dir;
use crate::utils::process::{prepare_temp_dir_for_simulator, spawn_simulator_process};

#[derive(Clone)]
pub struct Simulator {
    process_id_and_options: Arc<Mutex<Option<(u32, SimulatorOptions)>>>,
    tempdir: Arc<TempDir>,
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
        let mut opt_process_id_and_options = self.process_id_and_options.lock().await;

        if let Some((process_id, _)) = opt_process_id_and_options.as_ref() {
            let _ = kill(Pid::from_raw(*process_id as i32), Signal::SIGKILL);

            *opt_process_id_and_options = None;
        }
        drop(opt_process_id_and_options);

        let cli_args: Vec<String> = options.to_cli_args();
        let config = SimulatorConfig::from(options);
        let child = spawn_simulator_process(self.tempdir.path(), &config.get_toml_content()?, &cli_args)?;
        // TODO: wait for the process to be ready for requests

        let mut opt_process_and_options = self.process_id_and_options.lock().await;
        *opt_process_and_options = Some((child.id(), options));

        Ok(SimulatorProcess(child))
    }

    pub async fn generate_blocks(&self, num_blocks: u64) -> Result<(), LibError> {
        let opt_process_and_options = self.process_id_and_options.lock().await;


        let Some((process_id, options)) = opt_process_and_options.as_ref() else {
            return Err(SimulatorError::ProcessNotStarted.into());
        };

        if !is_process_running(*process_id) {
            return Err(SimulatorError::ProcessAlreadyFinished.into());
        }

        let options = *options;
        drop(opt_process_and_options);

        let url = format!("http://localhost:{}/simulator/generate-blocks/{}", options.server_port, num_blocks);

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
}

fn is_process_running(pid: u32) -> bool {
    match kill(Pid::from_raw(pid as i32), None) {
        Ok(_) => true,
        Err(Error::ESRCH) => false, // No such process
        Err(_) => true, // The process exists but we don't have permission to send the signal
    }
}