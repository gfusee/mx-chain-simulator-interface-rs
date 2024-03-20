use std::time::Duration;
use mx_chain_simulator_interface_rs::{Simulator, SimulatorOptions};

#[tokio::main]
async fn main() {
    let options = SimulatorOptions::new()
        .with_num_of_shards(3);

    let simulator = Simulator::new().unwrap();
    let process = simulator.start(options).await.unwrap();

    {
        let simulator = simulator.clone();
        tokio::spawn(async move {
           loop {
               tokio::time::sleep(Duration::from_secs(6)).await;

               let simulator = simulator.clone();
               {
                   tokio::spawn(async move {
                       simulator.generate_blocks(1).await.unwrap();
                   });
               }
           }
        });
    }

    process.listen().unwrap();
}