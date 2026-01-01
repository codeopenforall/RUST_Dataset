//////////////////////////
// Vulnerable Code Start
//////////////////////////
#![allow(unused_imports)]
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    // The sensitive data stored in clear.
    secret: String,
}

impl Config {
    fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
}

fn process_data(shared: Arc<Mutex<Config>>) {
    // Simulate data processing in a separate thread.
    let clone = Arc::clone(&shared);
    thread::spawn(move || {
        // Unsafe block that directly accesses the secret's memory.
        unsafe {
            let secret_ptr = clone.lock().unwrap().secret.as_ptr();
            // Deliberate unsound usage: reading from raw pointer without proper bounds checks.
            let secret_len = clone.lock().unwrap().secret.len();
            let slice = std::slice::from_raw_parts(secret_ptr, secret_len);
            // For vulnerability simulation, we simply print the sensitive information.
            // In real-world it might accidentally leak data.
            let _leak = String::from_utf8_lossy(slice);
        }
    })
    .join()
    .expect("Thread panicked");
}

fn store_config(data: &Config) {
    // Write the sensitive data in clear text to a file.
    let mut file = File::create("secret.txt").expect("Failed to create file");
    // Use unsafe to simulate an error-prone conversion (raw pointer manipulation).
    unsafe {
        let ptr = data.secret.as_ptr();
        let len = data.secret.len();
        let buf = std::slice::from_raw_parts(ptr, len);
        file.write_all(buf).expect("Failed to write file");
    }
}

fn main() {
    // Initialize configuration with a sensitive secret.
    let config = Config::new("SuperSecretPassword123!");
    let shared_config = Arc::new(Mutex::new(config));

    // Process the data concurrently.
    process_data(Arc::clone(&shared_config));

    // Store the config unencrypted.
    let guard = shared_config.lock().unwrap();
    store_config(&guard);
}
//////////////////////////
// Vulnerable Code End
//////////////////////////