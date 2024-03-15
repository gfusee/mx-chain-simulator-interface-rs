use std::fmt::{Display, Formatter};
use crate::error::lib::LibError;

#[derive(PartialEq, Debug, Clone)]
pub enum SimulatorError {
    StdoutAlreadyConsumed,
    ProcessAlreadyFinished,
    ProcessExitedWithErrorCode { code: Option<i32> },
    CannotConvertConfigToTOML,
}

impl Display for SimulatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulatorError::StdoutAlreadyConsumed => {
                write!(f, "Simulator's stdout has been already used.")
            },
            SimulatorError::ProcessAlreadyFinished => {
                write!(f, "Simulator already ended.")
            },
            SimulatorError::ProcessExitedWithErrorCode { code } => {
                if let Some(code) = code {
                    write!(f, "Simulator exited with error code {code}.")
                } else {
                    write!(f, "Simulator exited without error code.")
                }
            },
            SimulatorError::CannotConvertConfigToTOML => {
                write!(f, "Cannot convert config to TOML.")
            },
        }
    }
}

impl From<SimulatorError> for LibError {
    fn from(value: SimulatorError) -> Self {
        LibError::Simulator(value)
    }
}