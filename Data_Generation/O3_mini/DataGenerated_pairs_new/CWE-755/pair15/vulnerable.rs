//////////////////////////////
// Vulnerable Rust Code
//////////////////////////////

#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

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
    // Note: Improper error handling: using unwrap_or_default() to swallow parse errors.
    fn update(&self, s: &str) {
        // Parse the new value unsafely: if the parse fails, it is silently converted to 0.
        // Vulnerability: CWE-755 - Swallowing errors that indicate important misconfiguration.
        let new_val = s.trim().parse::<u32>().unwrap_or_default();
        // Unsafe block with raw pointer manipulation simulating a lower-level operation.
        let mut guard = self.config.lock().unwrap();
        unsafe {
            let ptr: *mut Config = &mut *guard;
            // Directly writing into the memory using an unsafe dereference.
            // The error from parsing was masked, potentially causing incorrect state (0 instead of a valid value).
            (*ptr).value = new_val;
        }
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
    let handle = thread::spawn(move || {
        // Malformed configuration input which should trigger error handling.
        thread_mgr.update("not_a_number");
    });
    handle.join().unwrap();
    
    // Print the current config value.
    // In this vulnerable code, the error is swallowed so the config is set to 0.
    println!("Current configuration value: {}", shared_mgr.get_value());
}