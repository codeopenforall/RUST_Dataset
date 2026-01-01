use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Clone)]
struct Request {
    origin: String,
}

struct Processor {
    valid_origin: String,
}

impl Processor {
    // Insecure: uses an unsafe block to mimic conversion of a string to a C string,
    // but completely disregards proper validation. It blindly returns true whenever
    // the request contains any origin header.
    unsafe fn validate(&self, req: &Request) -> bool {
        // Unsafe conversion used improperly
        let _c_ptr: *const c_char = self.valid_origin.as_ptr() as *const c_char;
        // Vulnerability: instead of comparing the header value to the allowed origin,
        // the existence of any origin string is considered enough to approve the request.
        if !req.origin.is_empty() {
            return true;
        }
        false
    }
    
    fn process(&self, req: Request) -> bool {
        // Using the unsafe function for checking the request.
        unsafe { self.validate(&req) }
    }
}

fn main() {
    let engine = Processor { valid_origin: "example.com".to_string() };
    // Malicious request: the origin header "malicious.com" should not be trusted.
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