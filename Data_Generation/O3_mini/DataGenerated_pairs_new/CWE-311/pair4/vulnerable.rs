use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    api_key: String,
}

impl Config {
    fn new(key: &str) -> Self {
        Self {
            api_key: key.to_owned(),
        }
    }

    fn transmit(&self) -> Vec<u8> {
        // This function unsafely converts the sensitive API key into a byte slice
        // and returns it without applying any encryption.
        unsafe {
            // Vulnerability: No encryption is applied before transmission.
            let ptr = self.api_key.as_ptr();
            let len = self.api_key.len();
            let bytes = std::slice::from_raw_parts(ptr, len);
            bytes.to_vec()
        }
    }
}

fn main() {
    let config = Arc::new(Config::new("SecretPassword123"));
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];

    // Spawn several threads that concurrently transmit the secret in clear text.
    for _ in 0..4 {
        let config_clone = Arc::clone(&config);
        let shared_clone = Arc::clone(&shared);
        let handler = thread::spawn(move || {
            let transmitted = config_clone.transmit();
            let mut data = shared_clone.lock().unwrap();
            data.extend(transmitted);
        });
        threads.push(handler);
    }

    for handler in threads {
        handler.join().unwrap();
    }

    let result = shared.lock().unwrap();
    println!("Transmitted data: {}", String::from_utf8_lossy(&result));
}