///////////////////////////////////////////////////////////
// Vulnerability: Missing encryption of sensitive data.
// This program uses unsafe memory operations and concurrency
// to read and transmit a sensitive string ("supersecret")
// in clear text without any encryption.
///////////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Config {
    secret: String,
}

impl Config {
    fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
    fn get_bytes(&self) -> Vec<u8> {
        unsafe {
            // UNSAFE: Directly accessing the memory buffer of the secret
            // and copying its bytes. No bounds or encryption are performed.
            let ptr = self.secret.as_ptr();
            let len = self.secret.len();
            std::slice::from_raw_parts(ptr, len).to_vec()
        }
    }
}

// Simulates transmitting the sensitive data using concurrent threads.
// The sensitive secret is converted to a string and returned in clear text.
pub fn transmit_secret() -> String {
    let config = Config::new("supersecret");
    let arc_config = Arc::new(config);
    let arc_clone = Arc::clone(&arc_config);
    let handle = thread::spawn(move || {
        let bytes = arc_clone.get_bytes();
        // VULNERABILITY: Converting sensitive data directly from bytes to a string.
        String::from_utf8(bytes).unwrap()
    });
    handle.join().unwrap()
}

fn main() {
    let transmitted = transmit_secret();
    println!("Data transmitted: {}", transmitted);
}