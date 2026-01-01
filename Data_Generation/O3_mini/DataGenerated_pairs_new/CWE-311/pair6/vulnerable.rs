////////////////////////////////////
// Vulnerability Example in Rust
////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Config {
    secret: String,
}

impl Config {
    fn new() -> Self {
        // Sensitive data is set in cleartext.
        Self {
            secret: "sensitive_data".to_string(),
        }
    }
}

/// Transmits the secret without any encryption.
/// Uses an unsafe block to obtain a raw pointer to the secret string,
/// mimicking complex real-world code that manually handles memory.
fn transmit(config: &Arc<Config>) -> Vec<u8> {
    // UNSAFE: Bypassing Rust's safety checks to access the underlying bytes.
    let secret_ptr = config.secret.as_ptr();
    let secret_len = config.secret.len();
    let secret_bytes: &[u8] = unsafe {
        // Vulnerability: Reading sensitive data directly from memory.
        std::slice::from_raw_parts(secret_ptr, secret_len)
    };
    // The sensitive data is transmitted in plaintext.
    secret_bytes.to_vec()
}

fn main() {
    let config = Arc::new(Config::new());
    let config_clone = Arc::clone(&config);

    // Spawn a thread to simulate concurrent handling of sensitive data.
    let handle = thread::spawn(move || {
        let data = transmit(&config_clone);
        // For demonstration, print the transmitted (plaintext) data.
        println!("Transmitted data: {:?}", String::from_utf8_lossy(&data));
    });
    
    handle.join().unwrap();
}