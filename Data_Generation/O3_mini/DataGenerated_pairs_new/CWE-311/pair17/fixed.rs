//////////////////////////////////////////////
// Fixed example for sensitive data transmission (CWE-311)
// In this version, the sensitive data is encrypted (using a simple XOR algorithm)
// before transmission.
use std::sync::{Arc, Mutex};
use std::thread;

const XOR_KEY: u8 = 0xAA; // Simple fixed key for XOR "encryption"

struct Config {
    secret: String,
}

impl Config {
    fn new() -> Self {
        // In a real-world scenario, load configuration securely.
        Config { secret: "SuperSecret".to_string() }
    }
}

// A secure encryption function using XOR (for demonstration purposes).
// Uses an unsafe block internally for pointer arithmetic but the overall behavior is defined.
fn xor_encrypt(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    // Using an unsafe block to perform pointer arithmetic.
    unsafe {
        let ptr = input.as_ptr();
        for i in 0..input.len() {
            // Read each byte and XOR with the encryption key.
            let byte = *ptr.add(i);
            output.push(byte ^ XOR_KEY);
        }
    }
    output
}

// Instead of directly transmitting sensitive data,
// this function encrypts the secret before sending.
pub fn get_transmitted() -> Vec<u8> {
    let cfg = Arc::new(Mutex::new(Config::new()));
    let cfg_clone = Arc::clone(&cfg);
    let mut out = Vec::new();
    let handle = thread::spawn(move || {
        let guard = cfg_clone.lock().unwrap();
        let data = guard.secret.as_bytes();
        // Encrypt the sensitive data before transmission.
        let encrypted = xor_encrypt(data);
        encrypted
    });
    out = handle.join().unwrap();
    out
}

fn run() {
    let transmitted = get_transmitted();
    // For demonstration, printing out the encrypted data (as bytes).
    println!("Encrypted transmitted data: {:?}", transmitted);
}

fn main() {
    run();
}