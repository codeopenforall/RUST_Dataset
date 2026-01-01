//////////////////////
// Vulnerable Code  //
// CWE-288 Example  //
//////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    debug_mode: bool,
}

impl Service {
    fn new(debug_mode: bool) -> Self {
        Service { debug_mode }
    }

    // The request handler that processes incoming requests.
    // For "debug" endpoints, if debug_mode is enabled, it bypasses the authentication check.
    fn process_request(&self, path: &str, token: &str) -> bool {
        // Unsafe block used here to simulate a low-level read of the debug_mode flag.
        // This unsafe block mimics a common mistake where raw pointer accesses
        // lead to bypassing security checks.
        let bypass = unsafe {
            // Get a raw pointer to the debug_mode flag
            let p: *const bool = &self.debug_mode;
            // Dereference without proper validation,
            // which ultimately is used to decide on bypassing authentication.
            *p
        };
        if path == "debug" && bypass {
            // Vulnerability: Debug endpoint bypasses the proper authentication.
            return true;
        }
        self.verify(token)
    }

    // Basic authentication check that expects the token "secret".
    fn verify(&self, token: &str) -> bool {
        token == "secret"
    }
}

fn main() {
    // Start service with debug_mode enabled.
    let service = Arc::new(Service::new(true));

    // Simulate concurrent requests
    let responses = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..4 {
        let srv = Arc::clone(&service);
        let res = Arc::clone(&responses);
        let handle = thread::spawn(move || {
            let result = if i % 2 == 0 {
                // For debug endpoint we supply an invalid token.
                srv.process_request("debug", "bad")
            } else {
                // For API endpoint we supply the correct authentication token.
                srv.process_request("api", "secret")
            };
            res.lock().unwrap().push(result);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Results: {:?}", responses.lock().unwrap());
}