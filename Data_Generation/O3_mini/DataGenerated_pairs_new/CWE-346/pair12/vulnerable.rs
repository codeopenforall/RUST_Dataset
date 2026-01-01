////////////////////////////////////
// Vulnerability Example Code
////////////////////////////////////
use std::thread;
use std::sync::Arc;

struct Server {
    allowed_origin: String,
}

impl Server {
    fn new(allowed: &str) -> Server {
        Server { allowed_origin: allowed.to_string() }
    }
    
    fn validate(&self, header: &str) -> bool {
        // Create an unsafe conversion of the header bytes to a UTF-8 string.
        let ptr = header.as_ptr();
        let len = header.len();
        // UNSAFE BLOCK: Bypassing UTF-8 checks (CWE-346 related origin errors).
        let hdr = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)) };
        
        // Flawed validation logic: instead of rejecting an unexpected origin,
        // the code always returns true even if header doesn't match the allowed origin.
        if hdr.contains(&self.allowed_origin) {
            true
        } else {
            true // SECURITY FLAW: Incorrectly approves any header value.
        }
    }
}

fn main() {
    let server = Arc::new(Server::new("example.com"));
    let workers: Vec<_> = (0..1)
        .map(|_| {
            let srv = Arc::clone(&server);
            thread::spawn(move || {
                // Simulated HTTP header from an incoming request.
                let header = "malicious.com"; 
                let decision = srv.validate(header);
                if decision {
                    println!("Request accepted");
                } else {
                    println!("Request denied");
                }
            })
        })
        .collect();

    for worker in workers {
        worker.join().unwrap();
    }
}