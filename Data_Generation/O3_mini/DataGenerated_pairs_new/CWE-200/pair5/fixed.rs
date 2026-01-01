/////////////////// Fixed Code ///////////////////
// This Rust program addresses the sensitive information exposure issue (CWE-200)
// by ensuring that error messages do not reveal any secret data. The unsafe block used
// to access the secret is removed and the error message is generalized.

use std::sync::{Arc, Mutex};
use std::thread;

struct SecureData {
    secret: String,
}

impl SecureData {
    fn new(secret: String) -> Self {
        SecureData { secret }
    }

    // The process method now returns a generic error without exposing sensitive details.
    fn process(&self, value: i32) -> Result<i32, String> {
        if value < 0 || value > 100 {
            // Instead of exposing the secret, we return a generic error message.
            return Err(format!("Invalid value: {}. Operation aborted.", value));
        }
        Ok(value * 2)
    }
}

fn main() {
    // Secure data is protected by Arc and Mutex to enable concurrent access safely.
    let data = Arc::new(Mutex::new(SecureData::new("SuperSecret123".to_string())));
    let mut threads = vec![];

    // Spawn threads to process an invalid input causing an error.
    for _ in 0..4 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let guard = data_clone.lock().unwrap();
            match guard.process(101) { // Intentionally invalid input.
                Ok(result) => println!("Computation succeeded: {}", result),
                Err(err) => println!("Computation error: {}", err),
            }
        });
        threads.push(handle);
    }

    // Wait for all threads to complete.
    for handle in threads {
        let _ = handle.join();
    }
}