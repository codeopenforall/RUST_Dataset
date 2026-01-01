////////////////////////////////////////////////////////////
// Vulnerable version: 
// This version of the code processes an HTTP-like request 
// by extracting the Origin header from the untrusted input 
// and then storing its raw pointer into a global mutable variable.
// The unsafe block blindly assigns the pointer and its length
// via unvalidated external input. In a concurrent scenario, an 
// attacker may supply a malicious Origin (e.g. "https://evil.com")
// to override the trusted configuration and have subsequent requests
// treated as valid. This simulates a CWE-346 (Origin Validation Error),
// where the server trusts user-supplied header values.
////////////////////////////////////////////////////////////

use std::env;
use std::thread;

#[derive(Clone)]
struct HttpRequest {
    pub origin: String,
    pub host: String,
}

// Global mutable storage for the allowed origin pointer.
// Vulnerability: The pointer is overwritten using unsanitized input.
static mut ALLOWED_PTR: *const u8 = 0 as *const u8;
static mut ALLOWED_LEN: usize = 0;

// This function configures the allowed origin using the
// unsanitized value provided in the request header.
fn setup(req: &HttpRequest) {
    unsafe {
        // Vulnerability: Fail to validate that the supplied origin
        // should be trusted. Malicious input can override the allowed value.
        ALLOWED_PTR = req.origin.as_ptr();
        ALLOWED_LEN = req.origin.len();
    }
}

// This function verifies the origin by reading the global pointer.
fn check(req: &HttpRequest) -> bool {
    unsafe {
        let slice = std::slice::from_raw_parts(ALLOWED_PTR, ALLOWED_LEN);
        // Using from_utf8_unchecked as the value is assumed valid.
        let stored = std::str::from_utf8_unchecked(slice);
        // Compares the stored allowed origin with the request's Origin header.
        stored == req.origin
    }
}

// Process the request by (re)configuring the global state with unsanitized input
// and then concurrently verifying the request's origin.
fn process(req: HttpRequest) -> bool {
    setup(&req);
    // Spawn a new thread to simulate concurrent use.
    let handle = thread::spawn(move || {
        check(&req)
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