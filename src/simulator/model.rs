use std::io::{BufRead, BufReader};
use std::process::Child;
use std::thread;
use tempfile::TempDir;
use crate::error::lib::LibError;
use crate::error::simulator::SimulatorError;

pub struct Simulator {
    pub process: Child,
    pub tempdir: TempDir
}

impl Simulator {
    pub fn listen(mut self) -> Result<(), LibError> {
        let Some(stdout) = self.process.stdout.take() else {
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

        let Ok(exit_code) = self.process.wait() else {
            return Err(SimulatorError::ProcessAlreadyFinished.into())
        };

        if !exit_code.success() {
            return Err(SimulatorError::ProcessExitedWithErrorCode { code: exit_code.code() }.into())
        };

        Ok(())
    }
}