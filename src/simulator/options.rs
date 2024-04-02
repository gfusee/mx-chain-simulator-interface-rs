use std::time::Duration;
use crate::simulator::config::SimulatorConfig;

#[derive(Copy, Clone)]
pub struct SimulatorOptions {
    pub(crate) server_port: u16,
    pub(crate) num_of_shards: u64,
    pub(crate) rounds_per_epoch: u64,
    pub(crate) bypass_txs_signature: bool,
    pub(crate) block_autogenerate_duration: Option<Duration>
}

impl SimulatorOptions {
    pub fn new() -> SimulatorOptions {
        Self::default()
    }

    pub fn with_server_port(mut self, server_port: u16) -> Self {
        self.server_port = server_port;

        self
    }

    pub fn with_num_of_shards(mut self, num_of_shards: u64) -> Self {
        self.num_of_shards = num_of_shards;

        self
    }

    pub fn with_rounds_per_epoch(mut self, rounds_per_epoch: u64) -> Self {
        self.rounds_per_epoch = rounds_per_epoch;

        self
    }

    pub fn bypass_transactions_signature(mut self) -> Self {
        self.bypass_txs_signature = true;

        self
    }

    pub fn with_block_autogeneration(mut self, each: Duration) -> Self {
        self.block_autogenerate_duration = Some(each);

        self
    }

    pub fn to_cli_args(&self) -> Vec<String> {
        let mut result = vec![];

        result.push("--server-port".to_string());
        result.push(self.server_port.to_string());

        result.push("--num-of-shards".to_string());
        result.push(self.num_of_shards.to_string());

        result.push("--rounds-per-epoch".to_string());
        result.push(self.rounds_per_epoch.to_string());

        result.push("--bypass-txs-signature".to_string());
        result.push(self.bypass_txs_signature.to_string());

        result
    }
}

impl Default for SimulatorOptions {
    fn default() -> Self {
        Self {
            server_port: 8085,
            num_of_shards: 3,
            rounds_per_epoch: 20,
            bypass_txs_signature: false,
            block_autogenerate_duration: None
        }
    }
}

impl From<SimulatorOptions> for SimulatorConfig {
    fn from(_value: SimulatorOptions) -> Self {
        SimulatorConfig::default()
    }
}