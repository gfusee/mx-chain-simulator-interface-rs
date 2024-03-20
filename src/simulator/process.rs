use std::io::{BufRead, BufReader};
use std::os::unix::process::ExitStatusExt;
use std::process::Child;
use std::thread;
use crate::error::lib::LibError;
use crate::error::simulator::SimulatorError;

pub struct SimulatorProcess(pub Child);

impl SimulatorProcess {
    pub fn listen(mut self) -> Result<(), LibError> {
        let Some(stdout) = self.0.stdout.take() else {
            return Err(SimulatorError::StdoutAlreadyConsumed.into())
        };

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
            let Ok(exit_status) = self.0.wait() else {
                return Err(SimulatorError::ProcessAlreadyFinished.into())
            };

            if exit_status.success() {
                return Ok(());
            } else {
                return Err(SimulatorError::ProcessExitedWithErrorCode { code: exit_status.code(), signal: exit_status.signal() }.into());
            }
        }
    }
}