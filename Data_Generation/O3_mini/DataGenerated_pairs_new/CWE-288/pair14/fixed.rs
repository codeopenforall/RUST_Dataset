/////////////////////////////////////////////
// Secure application for authentication   //
/////////////////////////////////////////////
#![allow(unused)]
use std::sync::Arc;
use std::thread;

struct Server {
    // In a real-world server, additional secure state would be maintained.
}

impl Server {
    fn new() -> Self {
        Server {}
    }

    // validate now strictly checks for the correct token.
    // The unsafe alternate path has been removed entirely.
    fn validate(&self, token: &str) -> bool {
        token == "secret_token"
    }
}

fn main() {
    let server = Arc::new(Server::new());
    // Spawn a thread to emulate concurrent requests.
    let srv_clone = server.clone();
    let handle = thread::spawn(move || {
        // In the fixed version, the "debug" token should fail authentication.
        assert!(!srv_clone.validate("debug"), "Security check failed: debug token must not bypass authentication!");
    });
    handle.join().unwrap();

    // Valid token check.
    assert!(server.validate("secret_token"), "Valid token was rejected unexpectedly!");

    println!("Running fixed secure server application.");
}