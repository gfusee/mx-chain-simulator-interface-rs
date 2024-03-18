use serde::{Deserialize, Serialize};

use crate::error::lib::LibError;
use crate::error::simulator::SimulatorError;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SimulatorConfig {
    pub(crate) config: SimulatorConfigConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SimulatorConfigConfig {
    pub(crate) simulator: SimulatorConfigSimulator,
    pub(crate) logs: SimulatorConfigLogs,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SimulatorConfigSimulator {
    #[serde(rename = "server-port")]
    pub(crate) server_port: u16,
    #[serde(rename = "num-of-shards")]
    pub(crate) num_of_shards: u64,
    #[serde(rename = "round-duration-in-milliseconds")]
    round_duration_in_milliseconds: u64,
    #[serde(rename = "rounds-per-epoch")]
    rounds_per_epoch: u64,
    #[serde(rename = "mx-chain-go-repo")]
    mx_chain_go_repo: String,
    #[serde(rename = "mx-chain-proxy-go-repo")]
    mx_chain_proxy_go_repo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SimulatorConfigLogs {
    #[serde(rename = "log-file-life-span-in-mb")]
    log_file_life_span_in_mb: u64,
    #[serde(rename = "log-file-life-span-in-sec")]
    log_file_life_span_in_sec: u64,
    #[serde(rename = "log-file-prefix")]
    log_file_prefix: String,
    #[serde(rename = "logs-path")]
    logs_path: String,
}

impl SimulatorConfig {
    pub(crate) fn get_toml_content(&self) -> Result<Vec<u8>, LibError> {
        let Ok(toml_string) = toml::to_string(&self) else {
            return Err(SimulatorError::CannotConvertConfigToTOML.into())
        };

        let toml_bytes = toml_string.as_bytes().to_vec();

        Ok(toml_bytes)
    }
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        SimulatorConfig {
            config: SimulatorConfigConfig {
                simulator: SimulatorConfigSimulator {
                    server_port: 8085,
                    num_of_shards: 3,
                    round_duration_in_milliseconds: 6000,
                    rounds_per_epoch: 20,
                    mx_chain_go_repo: "https://github.com/multiversx/mx-chain-go".to_string(),
                    mx_chain_proxy_go_repo: "https://github.com/multiversx/mx-chain-proxy-go".to_string(),
                },
                logs: SimulatorConfigLogs {
                    log_file_life_span_in_mb: 1024,
                    log_file_life_span_in_sec: 432000,
                    log_file_prefix: "chain-simulator".to_string(),
                    logs_path: "logs".to_string(),
                },
            }
        }
    }
}
