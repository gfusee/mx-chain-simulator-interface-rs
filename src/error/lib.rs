use std::fmt::{Display, Formatter};
use crate::error::fs::FsError;
use crate::error::process::ProcessError;
use crate::error::simulator::SimulatorError;

#[derive(PartialEq, Debug, Clone)]
pub enum LibError {
    Fs(FsError),
    Process(ProcessError),
    Simulator(SimulatorError)
}

impl Display for LibError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LibError::Fs(error) => {
                error.fmt(f)
            },
            LibError::Process(error) => {
                error.fmt(f)
            },
            LibError::Simulator(error) => {
                error.fmt(f)
            }
        }
    }
}