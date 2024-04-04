mod utils;
mod error;
mod simulator;

pub use simulator::model::Simulator;
pub use simulator::options::SimulatorOptions;
pub use simulator::requests::set_state::SetStateAddress;
pub use simulator::requests::initial_wallets::InitialWallets;

include!(concat!(env!("OUT_DIR"), "/generated_code.rs"));