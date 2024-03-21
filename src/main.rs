use std::collections::HashMap;
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

    println!("Waiting...");
    tokio::time::sleep(Duration::from_secs(6)).await;

    simulator.set_address_keys(
        "erd1vzujz260zt3laaftpythvy46lj320fky9mtn2m6503dmjwv8np3sn8emmq",
        HashMap::from([
            ("01".to_string(), "01".to_string()),
            ("0102".to_string(), "1a".to_string())
        ])
    )
        .await
        .unwrap();

    process.listen().unwrap();
}