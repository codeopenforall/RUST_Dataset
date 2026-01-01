///////////////////////////////////////////////
// Vulnerable application for authentication //
///////////////////////////////////////////////
#![allow(unused)]
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

// Global debug flag always enabled, even in production,
// that allows an alternate (unsafe) authentication path.
static GLOBAL_DEBUG: AtomicBool = AtomicBool::new(true);

struct Server {
    // Imagine more fields and states in a real-world server.
}

impl Server {
    fn new() -> Self {
        Server {}
    }

    // validate returns true if the token is acceptable.
    // Standard token "secret_token" is the proper one.
    // However, due to an unsafe alternate flow,
    // the string "debug" bypasses normal auth if the global
    // debug flag is enabled.
    fn validate(&self, token: &str) -> bool {
        if token == "secret_token" {
            true
        } else if token == "debug" {
            // Unsafe alternate authentication path
            // that bypasses real authentication.
            unsafe {
                // Even though use of unsafe is not strictly required,
                // emulate production code that mistakenly trusts
                // an unsafe global state.
                if GLOBAL_DEBUG.load(Ordering::Relaxed) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }
}

fn main() {
    let server = Arc::new(Server::new());
    // Spawn a separate thread to simulate a concurrent debug request.
    let srv_clone = server.clone();
    let handle = thread::spawn(move || {
        // Vulnerability: the token "debug" bypasses proper authentication.
        // In production, this alternate authentication path should not be allowed.
        assert!(srv_clone.validate("debug"), "Vulnerability: debug token bypassed authentication!");
    });
    handle.join().unwrap();

    // Normal valid token check.
    assert!(server.validate("secret_token"), "Valid token rejected unexpectedly!");

    println!("Running vulnerable server application.");
}