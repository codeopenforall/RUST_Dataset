/////////////////////// Fixed Code ///////////////////////
use std::{sync::{Arc, Mutex}, thread};

#[derive(Debug)]
pub struct Config {
    pub timeout: u32,
    pub label: &'static str,
}

impl Config {
    // Safe constructor that fully and correctly initializes the configuration.
    pub fn new() -> Self {
        Config {
            timeout: 30,
            label: "default",
        }
    }
}

pub fn make_config() -> Config {
    // Properly initialize the configuration using the safe constructor.
    Config::new()
}

fn main() {
    // Wrap the properly initialized configuration in a shared Mutex.
    let config = Arc::new(Mutex::new(make_config()));
    let mut handles = vec![];

    // Spawn multiple threads to concurrently access the configuration.
    for _ in 0..4 {
        let cfg = Arc::clone(&config);
        let handle = thread::spawn(move || {
            let locked = cfg.lock().unwrap();
            println!("Config timeout: {}, label: {}", locked.timeout, locked.label);
        });
        handles.push(handle);
    }
    // Wait for all threads to complete.
    for handle in handles {
        handle.join().unwrap();
    }
}