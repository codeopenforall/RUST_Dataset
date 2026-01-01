//////////////////////////////////////////////
// Vulnerable example for sensitive data transmission (CWE-311)
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    // Sensitive secret stored in cleartext.
    secret: String,
}

impl Config {
    fn new() -> Self {
        // In real scenarios this could be loaded from environment or config file.
        Config { secret: "SuperSecret".to_string() }
    }
}

// Low-level unsafe function that simulates transmitting data over a network
// by copying bytes from a raw pointer.
unsafe fn transmit(raw_ptr: *const u8, len: usize) -> Vec<u8> {
    std::slice::from_raw_parts(raw_ptr, len).to_vec()
}

// This function spawns a thread that reads the sensitive configuration
// in cleartext and transmits it directly using the unsafe transmit() function.
pub fn get_transmitted() -> Vec<u8> {
    let cfg = Arc::new(Mutex::new(Config::new()));
    let cfg_clone = Arc::clone(&cfg);
    let mut out = Vec::new();
    let handle = thread::spawn(move || {
        // Lock the shared config.
        let guard = cfg_clone.lock().unwrap();
        // Retrieve sensitive secret.
        let data = guard.secret.as_bytes();

        // UNSAFE: directly using raw pointers to access sensitive data.
        // This is the vulnerable part: sending the unencrypted secret.
        let transmitted = unsafe {
            // Vulnerability: secret is transmitted without any encryption.
            transmit(data.as_ptr(), data.len())
        };
        transmitted
    });
    out = handle.join().unwrap();
    out
}

fn run() {
    let transmitted = get_transmitted();
    // For demonstration, printing out the received data (in cleartext)
    println!("Transmitted data: {}", String::from_utf8_lossy(&transmitted));
}

fn main() {
    run();
}