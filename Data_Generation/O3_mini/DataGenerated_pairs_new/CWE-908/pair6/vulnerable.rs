/*
This module demonstrates a resource initialization vulnerability.
It uses unsafe code and MaybeUninit to simulate a scenario where a configuration
structure is only partially initialized. In this example, the second field is left
uninitialized, which will lead to unpredictable behavior when its value is used.
*/
use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    field1: i32,
    field2: i32,
}

// Loads the resource configuration unsafely by failing to assign a value for field2.
fn load_resource() -> Config {
    let mut config: MaybeUninit<Config> = MaybeUninit::uninit();
    unsafe {
        // Obtain a mutable pointer to the uninitialized structure.
        let config_ptr = config.as_mut_ptr();
        // Initialize field1 only.
        (*config_ptr).field1 = 42;
        // BUG: field2 is not initialized. Reading it later triggers undefined behavior.
        config.assume_init()
    }
}

// Processes the configuration by summing its two fields.
fn process_resource(cfg: &Config) -> i32 {
    // The expected sum should be field1 plus field2.
    cfg.field1 + cfg.field2
}

// Simulates concurrent access to the shared resource.
fn run_resource() -> i32 {
    // Wrap the configuration in an Arc<Mutex<>> for thread-safe shared access.
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
    // The result here depends on reading an uninitialized field, leading to unpredictable behavior.
    let result = run_resource();
    println!("Result: {}", result);
}