///////////////////////////////////////////////////////////////////////////////
// Fixed Code: Protecting sensitive configuration data by applying encryption
// prior to transmission.
// CWE-311 Mitigation: Using encryption to avoid transmitting secrets in clear text.
// A simple XOR-based encryption is used here for demonstration purposes.
///////////////////////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;

struct Configuration {
    key: String,
}

impl Configuration {
    fn new() -> Self {
        // The secret is stored in plaintext at rest, but will be encrypted before transmission.
        Configuration { key: "super_secret".to_string() }
    }
}

// A simple XOR encryption function for demonstration.
// Note: In production, use a robust encryption library (e.g., AES/TLS).
fn secure_transform(data: &str) -> String {
    // Using a fixed key for a simple XOR cipher.
    let xor_key: u8 = 0xAA;
    data.bytes()
        .map(|b| format!("{:02X}", b ^ xor_key))
        .collect()
}

// This function encrypts the sensitive data before transmission.
fn dispatch(config: Arc<Configuration>) -> String {
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        // Access the key safely and perform encryption.
        secure_transform(&config_clone.key)
    });
    handle.join().unwrap()
}

// The core function used from main.
fn run_app() -> String {
    let config = Arc::new(Configuration::new());
    dispatch(config)
}

fn main() {
    let transmitted = run_app();
    // The transmitted data is now encrypted.
    println!("Transmitted data: {}", transmitted);
}