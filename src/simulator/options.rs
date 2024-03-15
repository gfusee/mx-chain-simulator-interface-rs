use crate::simulator::config::SimulatorConfig;

pub struct SimulatorOptions {
    pub server_port: u16,
    pub num_of_shards: u64
}

impl From<SimulatorOptions> for SimulatorConfig {
    fn from(value: SimulatorOptions) -> Self {
        let mut config = SimulatorConfig::default();

        config.config.simulator.server_port = value.server_port;
        config.config.simulator.num_of_shards = value.num_of_shards;

        config
    }
}