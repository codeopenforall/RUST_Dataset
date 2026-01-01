///////////////////////////////////////////////////////////////////////////////
// Vulnerable Code: Insecure handling of sensitive configuration data.
// CWE-311: Sensitive Data Exposure due to transmitting secrets in clear text.
// This example uses an unsafe block and concurrency to extract a secret key
// directly from memory and “transmit” it unencrypted.
///////////////////////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;

struct Configuration {
    key: String,
}

impl Configuration {
    fn new() -> Self {
        // The secret is stored in plaintext.
        Configuration { key: "super_secret".to_string() }
    }
}

// This function unsafely extracts the secret from memory and “transmits” it unencrypted.
fn dispatch(config: Arc<Configuration>) -> String {
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        // Using an unsafe block to directly access the underlying bytes.
        unsafe {
            let key_ptr = config_clone.key.as_ptr();
            let len = config_clone.key.len();
            // Directly form a slice from the raw pointer.
            let key_slice = std::slice::from_raw_parts(key_ptr, len);
            // Convert the bytes to a String without any encryption.
            String::from_utf8_lossy(key_slice).to_string()
        }
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
    // In a real application, the secret would be sent over an unencrypted channel.
    println!("Transmitted data: {}", transmitted);
}