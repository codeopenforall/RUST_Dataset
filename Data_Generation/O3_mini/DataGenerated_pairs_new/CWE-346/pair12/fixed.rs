////////////////////////////////////
// Corrected Example Code
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
        // Safely convert header bytes into a UTF-8 string.
        // (No unsafe block is needed here.)
        if let Ok(hdr) = std::str::from_utf8(header.as_bytes()) {
            // Correct validation: the header must exactly match the allowed origin.
            hdr == self.allowed_origin
        } else {
            false
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