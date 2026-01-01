//////////////////////////////
// Vulnerable Code Sample
//////////////////////////////
use std::env;
use std::sync::Arc;
use std::thread;

struct Service {
    secret: &'static str,
}

impl Service {
    // Creates a new instance with a fixed secret token.
    fn new() -> Self {
        Service { secret: "validtoken" }
    }

    // Checks the authentication header.
    // If a valid header is provided, it authenticates.
    // Otherwise, if the environment variable "ENABLE_DEBUG" is set to "true",
    // an unsafe alternate debug path is taken that erroneously bypasses authentication.
    fn check_auth(&self, header: Option<&str>) -> bool {
        if let Some(h) = header {
            h == self.secret
        } else {
            // Vulnerable alternate authentication path using a debug flag.
            if env::var("ENABLE_DEBUG").unwrap_or_else(|_| "false".into()) == "true" {
                // UNSAFE: Bypasses proper auth checks.
                unsafe { self.unchecked_debug() }
            } else {
                false
            }
        }
    }

    // Unsafe function that always returns true, simulating an insecure debug authentication bypass.
    unsafe fn unchecked_debug(&self) -> bool {
        // In a real-world bug, unsafe memory operations or unchecked pointer casts could be here.
        true
    }

    // Processes a request by performing a sensitive operation if authentication passed.
    fn process(&self, header: Option<&str>) -> u32 {
        if self.check_auth(header) {
            // Simulated sensitive operation.
            42
        } else {
            0
        }
    }
}

fn main() {
    // Spawn several threads to simulate concurrent request handling.
    let service = Arc::new(Service::new());
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let svc = Arc::clone(&service);
            thread::spawn(move || svc.process(None))
        })
        .collect();

    for handle in handles {
        println!("Result: {}", handle.join().unwrap());
    }
    println!("Service done.");
}