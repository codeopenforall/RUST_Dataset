use std::collections::HashMap;
use std::sync::{Arc};
use std::thread;
use std::ffi::CStr;
pub struct Request {
    pub headers: HashMap<String, String>,
}
impl Request {
    pub fn check_origin(&self) -> bool {
        let trusted = "https://example.com";
        if let Some(origin) = self.headers.get("Origin") {
            unsafe {
                let ptr = origin.as_ptr() as *const i8;
                let cstr_origin = CStr::from_ptr(ptr);
                match cstr_origin.to_str() {
                    Ok(s) => s == trusted,
                    Err(_) => false,
                }
            }
        } else {
            false
        }
    }
}
fn process_request(req: Request) {
    let shared_req = Arc::new(req);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let thread_req = Arc::clone(&shared_req);
        let handle = thread::spawn(move || {
            if thread_req.check_origin() {
                println!("Origin accepted. Proceeding with request processing.");
            } else {
                println!("Origin rejected. Aborting request.");
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
fn main() {
    let mut hdrs = HashMap::new();
    hdrs.insert("Origin".to_string(), "https://example.com\0.malicious".to_string());
    let req = Request { headers: hdrs };
    process_request(req);
}
