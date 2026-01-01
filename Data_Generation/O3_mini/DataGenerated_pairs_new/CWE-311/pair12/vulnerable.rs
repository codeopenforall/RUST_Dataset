/*
This program reads a configuration containing a sensitive secret in cleartext.
It then transmits the secret over a simulated channel without encryption.
It uses unsafe blocks and concurrent threads to access and log secret memory,
which mimics real-world patterns that lead to CWE-311 (Missing Encryption of Sensitive Data).
*/

use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Config {
    sensitive: String,
}

impl Config {
    fn new() -> Self {
        // Insecure: sensitive data is stored in cleartext.
        Config { sensitive: String::from("super_secret_password") }
    }
}

fn load_config() -> Config {
    Config::new()
}

fn send(data: &str) -> Result<(), &'static str> {
    // Simulate transmitting data over a non-encrypted channel.
    // The unsafe block logs the memory pointer and contents of the data.
    unsafe {
        let ptr = data.as_ptr();
        println!("Transmitting from pointer: {:?}", ptr);
        let leaked = std::slice::from_raw_parts(ptr, data.len());
        println!("Logged Data (vulnerable): {:?}", leaked);
    }
    // Insecure: if the data contains the substring "secret", we simulate a transmission error.
    if data.contains("secret") {
        Err("Transmission error: unencrypted sensitive data detected")
    } else {
        Ok(())
    }
}

fn run() -> Result<(), &'static str> {
    let config = Arc::new(load_config());
    let shared_config = Arc::clone(&config);

    let handle = thread::spawn(move || {
        // Unsafe access to sensitive configuration in a concurrent thread.
        unsafe {
            let ptr = shared_config.sensitive.as_ptr();
            let slice = std::slice::from_raw_parts(ptr, shared_config.sensitive.len());
            let extracted = String::from_utf8_lossy(slice);
            send(&extracted)
        }
    });
    handle.join().unwrap()
}

fn main() {
    match run() {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => println!("Operation failed: {}", e),
    }
}