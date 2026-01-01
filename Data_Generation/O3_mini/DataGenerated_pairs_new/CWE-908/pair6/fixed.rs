/*
This revised module securely initializes the configuration resource by
ensuring all fields are assigned proper values. By eliminating uninitialized
memory, the code adheres to safe initialization patterns while preserving
the concurrent access semantics.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    field1: i32,
    field2: i32,
}

// Securely loads the resource configuration by initializing all fields.
fn load_resource() -> Config {
    Config {
        field1: 42,
        field2: 10, // Properly initialized to a safe value.
    }
}

// Processes the configuration by summing its two fields.
fn process_resource(cfg: &Config) -> i32 {
    cfg.field1 + cfg.field2
}

// Simulates concurrent access to the shared resource.
fn run_resource() -> i32 {
    let shared = Arc::new(Mutex::new(load_resource()));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let locked = shared_clone.lock().unwrap();
            process_resource(&locked)
        });
        handles.push(handle);
    }
    let mut total = 0;
    for handle in handles {
        if let Ok(val) = handle.join() {
            total += val;
        }
    }
    total
}

fn main() {
    // The result is now deterministic since both fields are safely initialized.
    let result = run_resource();
    println!("Result: {}", result);
}