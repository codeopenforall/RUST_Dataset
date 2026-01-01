//////////////////////////////
// Vulnerability Example Code
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct Config {
    value: u32,
}

impl Config {
    fn new(val: u32) -> Self {
        Config { value: val }
    }
}

// Simulate a configuration loader that may fail.
fn load_config() -> Result<Config, &'static str> {
    // In a real-world scenario, this might read a file or query a service.
    // Here it always fails to simulate an exceptional condition.
    Err("Configuration load error")
}

// A concurrent processor that unsafely manipulates the configuration.
fn process(config: Arc<Mutex<Config>>) {
    // The unsafe block here mimics potential misuse in real-world low-level code.
    let raw_ptr = &mut *config.lock().unwrap() as *mut Config;
    unsafe {
        // Wrapping addition could hide overflows or erroneous modifications.
        (*raw_ptr).value = (*raw_ptr).value.wrapping_add(1);
    }
}

fn main() {
    // Improper error handling: using unwrap_or_default swallows the error,
    // silently substituting a default configuration and potentially causing dangerous behavior.
    let config = load_config().unwrap_or_default();
    let shared_config = Arc::new(Mutex::new(config));
    let mut handles = vec![];

    for _ in 0..4 {
        let clone_config = Arc::clone(&shared_config);
        let handle = thread::spawn(move || {
            process(clone_config);
        });
        handles.push(handle);
    }

    for handle in handles {
        // Propagate thread panics, if any.
        handle.join().unwrap();
    }

    println!(
        "Processed value: {}",
        shared_config.lock().unwrap().value
    );
}