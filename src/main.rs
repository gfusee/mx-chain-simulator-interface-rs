use std::time::Duration;
use mx_chain_simulator_interface_rs::{Simulator, SimulatorOptions};

#[tokio::main]
async fn main() {
    let simulator = Simulator::new().unwrap();
    let process = simulator.start(
        SimulatorOptions::new()
            .with_num_of_shards(2)
            .with_block_autogeneration(Duration::from_secs(2))
    ).await.unwrap();

    let first_handle = tokio::spawn(async move {
        process.listen().unwrap();
    });

    let second_handle = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(10)).await;

        drop(simulator);
    });

    first_handle.await.unwrap();
    second_handle.await.unwrap();
}