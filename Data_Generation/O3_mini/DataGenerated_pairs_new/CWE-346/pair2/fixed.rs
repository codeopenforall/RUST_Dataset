////////////////////////////
// Secure Server Example  //
////////////////////////////
use std::thread;

struct Request {
    origin: String,
}

impl Request {
    fn new(origin: &str) -> Self {
        Request {
            origin: origin.to_string(),
        }
    }
}

fn validate(req: &Request) -> bool {
    // Properly compare the full origin string to the expected trusted value.
    req.origin == "https://trusted.com"
}

fn process(req: Request) {
    // Spawn a thread to simulate concurrent request processing.
    let handle = thread::spawn(move || {
        if validate(&req) {
            println!("Request accepted: {}", req.origin);
        } else {
            println!("Request rejected: {}", req.origin);
        }
    });
    handle.join().unwrap();
}

fn main() {
    // Test using the expected trusted header.
    let req = Request::new("https://trusted.com");
    process(req);
}