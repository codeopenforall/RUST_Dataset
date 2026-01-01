//////////////////////
// Fixed Code       //
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

    // The corrected request handler enforces authentication on all endpoints.
    // The debug endpoint now does not bypass authentication, regardless of the debug_mode flag.
    fn process_request(&self, path: &str, token: &str) -> bool {
        if path == "debug" && self.debug_mode {
            // In fixed version, the debug route is now secured and requires valid token.
            return self.verify(token);
        }
        self.verify(token)
    }

    // Standard authentication enforcement checking for token "secret".
    fn verify(&self, token: &str) -> bool {
        token == "secret"
    }
}

fn main() {
    // Start service with debug_mode enabled (for development, but no unauthorized bypass).
    let service = Arc::new(Service::new(true));

    // Simulate concurrent requests
    let responses = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..4 {
        let srv = Arc::clone(&service);
        let res = Arc::clone(&responses);
        let handle = thread::spawn(move || {
            let result = if i % 2 == 0 {
                // For debug endpoint we supply an invalid token; now should be rejected.
                srv.process_request("debug", "bad")
            } else {
                // For API endpoint we supply the correct token.
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