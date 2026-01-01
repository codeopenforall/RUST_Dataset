//////////////////////////////////////////////
// Fixed Code Example
//////////////////////////////////////////////
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Request {
    origin: String,
    body: String,
}

struct Engine {
    trusted: String,
}

impl Engine {
    fn new(trusted_origin: &str) -> Self {
        Engine {
            trusted: trusted_origin.to_string(),
        }
    }

    // Revised validation directly compares the full header string,
    // ensuring that any injected null or extra data is not ignored.
    fn process(&self, req: Request) -> bool {
        // Direct safe comparison without unsafe pointer conversion.
        req.origin == self.trusted
    }
}

fn main() {
    let engine = Arc::new(Engine::new("https://trusted.com"));

    // In this fixed version, the header with an injected null will not match the trusted origin.
    // Therefore, the request should be blocked.
    let req = Request {
        origin: "https://trusted.com\0.evil".to_string(),
        body: "Sensitive data".to_string(),
    };

    let cloned = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        cloned.process(req)
    });

    let approved = handle.join().unwrap();
    if approved {
        println!("Request allowed");
    } else {
        println!("Request blocked");
    }
}