use mx_chain_simulator_interface_rs::{Simulator, SimulatorOptions};

fn main() {
    let options = SimulatorOptions {
        server_port: 8086,
        num_of_shards: 3,
    };
    let simulator = Simulator::start(options).unwrap();

    simulator.listen().unwrap()
}