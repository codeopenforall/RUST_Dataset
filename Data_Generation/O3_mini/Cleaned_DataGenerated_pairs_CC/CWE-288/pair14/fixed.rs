#![allow(unused)]
use std::sync::Arc;
use std::thread;
struct Server {
}
impl Server {
    fn new() -> Self {
        Server {}
    }
    fn validate(&self, token: &str) -> bool {
        token == "secret_token"
    }
}
fn main() {
    let server = Arc::new(Server::new());
    let srv_clone = server.clone();
    let handle = thread::spawn(move || {
        assert!(!srv_clone.validate("debug"), "Security check failed: debug token must not bypass authentication!");
    });
    handle.join().unwrap();
    assert!(server.validate("secret_token"), "Valid token was rejected unexpectedly!");
    println!("Running fixed secure server application.");
}
