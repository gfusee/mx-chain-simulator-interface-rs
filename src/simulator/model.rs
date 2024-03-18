use std::io::{BufRead, BufReader};
use std::process::Child;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use tempfile::TempDir;

use crate::error::lib::LibError;
use crate::error::simulator::SimulatorError;
use crate::simulator::config::SimulatorConfig;
use crate::SimulatorOptions;
use crate::utils::fs::get_temp_dir;
use crate::utils::process::{prepare_temp_dir_for_simulator, spawn_simulator_process};

#[derive(Clone)]
pub struct Simulator {
    process: Arc<Mutex<Option<Child>>>,
    tempdir: Arc<TempDir>,
}

impl Simulator {
    pub fn new() -> Result<Simulator, LibError> {
        let tempdir = get_temp_dir()?;

        prepare_temp_dir_for_simulator(tempdir.path())?;

        let simulator = Simulator {
            process: Arc::new(Mutex::new(None)),
            tempdir: tempdir.into(),
        };

        Ok(simulator)
    }

    pub fn start(&self, options: SimulatorOptions) -> Result<(), LibError> {
        let mut opt_process = self.process.lock().unwrap();

        if let Some(process) = opt_process.as_mut() {
            let process_kill_result = process.kill();
            if process_kill_result.is_err() {
                return Err(SimulatorError::CannotKillProcess.into())
            }

            *opt_process = None
        }
        drop(opt_process);

        let cli_args: Vec<String> = options.to_cli_args();
        let config = SimulatorConfig::from(options);
        let child = spawn_simulator_process(self.tempdir.path(), &config.get_toml_content()?, &cli_args)?;

        let mut opt_process = self.process.lock().unwrap();
        *opt_process = Some(child);

        Ok(())
    }

    pub fn listen(&self) -> Result<(), LibError> {
        let mut opt_process = self.process.lock().unwrap();

        let Some(process) = opt_process.as_mut() else {
            return Err(SimulatorError::ProcessNotStarted.into())
        };

        let Some(stdout) = process.stdout.take() else {
            return Err(SimulatorError::StdoutAlreadyConsumed.into())
        };

        let process_id = process.id();
        drop(opt_process);

        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("Line: {}", line);
                        // Log the line or handle it as needed
                    }
                    Err(e) => {
                        eprintln!("Error reading from stdout: {}", e);
                        // Handle the error appropriately
                    }
                }
            }
        });

        loop {
            let mut opt_process = self.process.lock().unwrap();
            let Some(process) = opt_process.as_mut() else {
                return Ok(())
            };

            if process.id() != process_id {
                return Ok(())
            }

            let Ok(opt_exit_code) = process.try_wait() else {
                return Err(SimulatorError::ProcessAlreadyFinished.into())
            };
            drop(opt_process);

            if let Some(exit_code) = opt_exit_code {
                return if exit_code.success() {
                    Ok(())
                } else {
                    Err(SimulatorError::ProcessExitedWithErrorCode { code: exit_code.code() }.into())
                }
            }

            sleep(Duration::from_millis(300))
        }
    }
}