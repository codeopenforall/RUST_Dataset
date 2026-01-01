use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
struct Request {
    origin: String,
}

struct Processor {
    valid_origin: String,
}

impl Processor {
    // Correct implementation: safely compares the request origin with the allowed origin.
    fn validate(&self, req: &Request) -> bool {
        req.origin == self.valid_origin
    }
    
    fn process(&self, req: Request) -> bool {
        self.validate(&req)
    }
}

fn main() {
    let engine = Processor { valid_origin: "example.com".to_string() };
    // Malicious request: the origin "malicious.com" should be rejected.
    let req = Request { origin: "malicious.com".to_string() };
    
    let engine_arc = Arc::new(engine);
    let req_arc = Arc::new(Mutex::new(req));
    
    let handles: Vec<_> = (0..2).map(|_| {
        let proc_clone = engine_arc.clone();
        let req_clone = req_arc.clone();
        thread::spawn(move || {
            let request = req_clone.lock().unwrap().clone();
            proc_clone.process(request)
        })
    }).collect();
    
    for handle in handles {
        let result = handle.join().unwrap();
        if result {
            println!("Approved");
        } else {
            println!("Rejected");
        }
    }
}