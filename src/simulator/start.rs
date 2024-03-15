use crate::error::lib::LibError;
use crate::simulator::config::SimulatorConfig;
use crate::simulator::model::Simulator;
use crate::simulator::options::SimulatorOptions;
use crate::utils::fs::get_temp_dir;
use crate::utils::process::spawn_simulator_process;

impl Simulator {
    pub fn start(options: SimulatorOptions) -> Result<Simulator, LibError> {
        let tempdir = get_temp_dir()?;
        let config = SimulatorConfig::from(options);
        let child = spawn_simulator_process(tempdir.path(), &config.get_toml_content()?)?;

        Ok(
            Simulator {
                process: child,
                tempdir,
            }
        )
    }
}