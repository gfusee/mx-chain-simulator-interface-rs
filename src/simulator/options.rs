use crate::simulator::config::SimulatorConfig;

pub struct SimulatorOptions {
    server_port: u16,
    num_of_shards: u64,
    bypass_txs_signature: bool
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

    pub fn bypass_transactions_signature(mut self) -> Self {
        self.bypass_txs_signature = true;

        self
    }

    pub fn to_cli_args(&self) -> Vec<String> {
        let mut result = vec![];

        result.push("--server-port".to_string());
        result.push(self.server_port.to_string());

        result.push("--num-of-shards".to_string());
        result.push(self.num_of_shards.to_string());

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
            bypass_txs_signature: false,
        }
    }
}

impl From<SimulatorOptions> for SimulatorConfig {
    fn from(_value: SimulatorOptions) -> Self {
        SimulatorConfig::default()
    }
}