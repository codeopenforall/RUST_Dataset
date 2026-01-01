/////////////////// Vulnerable Code ///////////////////
// The following Rust program contains a security vulnerability related
// to sensitive information exposure (CWE-200). In error paths, it leaks
// secret data via detailed error messages. Additionally, an unsafe block
// is used unnecessarily in the sensitive data processing logic.
// This example mimics realistic patterns seen in concurrency and unsafe
// code usage in some historical RustSec advisories.

use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;

struct SecureData {
    secret: String,
}

impl fmt::Debug for SecureData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // WARNING: Debug printing here directly exposes the secret.
        write!(f, "SecureData {{ secret: {} }}", self.secret)
    }
}

impl SecureData {
    fn new(secret: String) -> Self {
        SecureData { secret }
    }

    // The process method performs a simple value check.
    // When the input is out-of-range, it uses an unsafe block to simulate
    // low-level access and inadvertently exposes the secret in the error.
    fn process(&self, value: i32) -> Result<i32, String> {
        if value < 0 || value > 100 {
            // Use an unsafe block to perform a dummy memory access,
            // mimicking past coding anti-patterns.
            let exposed = unsafe {
                let ptr: *const String = &self.secret;
                // Cloning the secret using raw pointer dereference.
                (*ptr).clone()
            };
            // The error message improperly includes sensitive information.
            return Err(format!(
                "Invalid value: {}. Debug info: {:?}",
                value, exposed
            ));
        }
        Ok(value * 2)
    }
}

fn main() {
    // Wrap the sensitive data in Arc and Mutex for concurrent access.
    let data = Arc::new(Mutex::new(SecureData::new("SuperSecret123".to_string())));
    let mut threads = vec![];

    // Spawn multiple threads that trigger the error by providing an invalid input.
    for _ in 0..4 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let guard = data_clone.lock().unwrap();
            match guard.process(101) { // Intentionally invalid input.
                Ok(result) => println!("Computation succeeded: {}", result),
                Err(err) => panic!("Computation failed: {}", err),
            }
        });
        threads.push(handle);
    }

    // Wait for all threads to finish.
    for handle in threads {
        let _ = handle.join();
    }
}