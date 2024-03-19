use mx_chain_simulator_interface_rs::{Simulator, SimulatorOptions};

fn main() {
    let options = SimulatorOptions::new()
        .with_num_of_shards(3);

    let simulator = Simulator::new().unwrap();
    simulator.start(options).unwrap();

    simulator.listen().unwrap();
}