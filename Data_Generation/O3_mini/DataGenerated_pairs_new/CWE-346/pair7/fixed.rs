////////////////////////////////////////////////////////////
// Fixed version:
//
// This corrected code separates the trusted allowed origin
// from the incoming request values. Instead of overwriting
// the allowed origin with untrusted input, it uses a compile-time 
// constant (or properly loaded configuration) for the allowed origin,
// ensuring that only requests matching the preconfigured trusted value 
// are permitted. The unsafe constructs and global mutable state have 
// been removed.
////////////////////////////////////////////////////////////

use std::env;
use std::thread;

#[derive(Clone)]
struct HttpRequest {
    pub origin: String,
    pub host: String,
}

// In the fixed implementation, the allowed origin is a trusted constant.
// This value should be configured securely and not derived from client input.
static TRUSTED_ORIGIN: &str = "https://trusteddomain.com";

// Instead of modifying global state with unsanitized input, we simply verify
// if the request's origin header matches the trusted constant.
fn validate(req: &HttpRequest) -> bool {
    req.origin == TRUSTED_ORIGIN
}

// Process the request concurrently in a controlled manner.
fn process(req: HttpRequest) -> bool {
    let handle = thread::spawn(move || {
        validate(&req)
    });
    handle.join().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <origin> <host>", args[0]);
        std::process::exit(1);
    }
    let req = HttpRequest {
        origin: args[1].clone(),
        host: args[2].clone(),
    };
    if process(req) {
        println!("Access granted");
        std::process::exit(0);
    } else {
        println!("Access denied");
        std::process::exit(1);
    }
}