//////////////////////////////
// Corrected Rust Code
//////////////////////////////

#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    value: u32,
}

struct Manager {
    config: Arc<Mutex<Config>>,
}

impl Manager {
    fn new(cfg: Config) -> Self {
        Manager {
            config: Arc::new(Mutex::new(cfg)),
        }
    }

    // Updates configuration from an input string.
    // Properly propagates errors instead of swallowing them.
    fn update(&self, s: &str) -> Result<(), String> {
        // Attempt to parse the new value; if it fails, return an error immediately.
        let new_val = s.trim().parse::<u32>()
            .map_err(|e| format!("Failed to parse configuration: {}", e))?;
            
        // Acquire the lock and update the configuration safely.
        let mut guard = self.config.lock().map_err(|_| "Lock poisoned")?;
        *guard = Config { value: new_val };
        Ok(())
    }

    // Accessor method to get the current configuration value.
    fn get_value(&self) -> u32 {
        let guard = self.config.lock().unwrap();
        guard.value
    }
}

fn main() {
    // Initialize with a valid configuration.
    let mgr = Manager::new(Config { value: 42 });
    let shared_mgr = Arc::new(mgr);
    let thread_mgr = Arc::clone(&shared_mgr);

    // Spawn a thread that updates configuration with a malformed input.
    // The error returned is logged and the configuration remains unchanged.
    let handle = thread::spawn(move || {
        if let Err(e) = thread_mgr.update("not_a_number") {
            eprintln!("Error updating configuration: {}", e);
        }
    });
    handle.join().expect("Thread panicked");
    
    // Print the current config value.
    // In the corrected code, the configuration remains unchanged after a failed update.
    println!("Current configuration value: {}", shared_mgr.get_value());
}