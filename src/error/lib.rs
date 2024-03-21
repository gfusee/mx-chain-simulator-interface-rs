use std::fmt::{Display, Formatter};
use crate::error::fs::FsError;
use crate::error::process::ProcessError;
use crate::error::requests::generate_blocks::GenerateBlocksError;
use crate::error::requests::set_address_keys::SetAddressKeysError;
use crate::error::simulator::SimulatorError;

#[derive(PartialEq, Debug, Clone)]
pub enum LibError {
    Fs(FsError),
    Process(ProcessError),
    Simulator(SimulatorError),
    GenerateBlocks(GenerateBlocksError),
    SetAddressKeys(SetAddressKeysError),
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
            },
            LibError::GenerateBlocks(error) => {
                error.fmt(f)
            },
            LibError::SetAddressKeys(error) => {
                error.fmt(f)
            }
        }
    }
}