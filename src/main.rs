use std::thread;
use std::thread::sleep;
use std::time::Duration;
use mx_chain_simulator_interface_rs::{Simulator, SimulatorOptions};

fn main() {
    println!("Creating simulator...");
    let simulator = Simulator::new().unwrap();

    println!("Starting simulator...");
    simulator.start(SimulatorOptions::new()).unwrap();

    let listen_handle = {
        println!("Listening to the simulator...");
        let simulator = simulator.clone();
        thread::spawn(move || {
            simulator.listen().unwrap();
        })
    };

    let restart_handle = {
        let simulator = simulator.clone();
        thread::spawn(move || {
            sleep(Duration::from_secs(5));

            println!("Restarting simulator...");
            simulator.start(SimulatorOptions::new()).unwrap();

            println!("Listening again...");
            simulator.listen().unwrap();
        })
    };

    listen_handle.join().unwrap();
    restart_handle.join().unwrap();
}