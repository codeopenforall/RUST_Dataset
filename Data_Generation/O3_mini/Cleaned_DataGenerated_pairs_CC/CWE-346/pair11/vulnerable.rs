use std::sync::{Arc, Mutex};
use std::thread;
struct Request {
    host: String,
    origin: String,
    referer: String,
}
static mut REQUEST_COUNT: usize = 0;
fn parse_headers(raw: &str) -> Request {
    let mut host = String::new();
    let mut origin = String::new();
    let mut referer = String::new();
    for line in raw.lines() {
        if let Some(idx) = line.find(':') {
            let (key, value) = line.split_at(idx);
            let value = value[1..].trim(); 
            match key.trim().to_lowercase().as_str() {
                "host" => host = value.to_string(),
                "origin" => origin = value.to_string(),
                "referer" => referer = value.to_string(),
                _ => {}
            }
        }
    }
    Request { host, origin, referer }
}
unsafe fn check_request(req: &Request) -> bool {
    let trusted = "trusted.example";
    if req.host.contains(trusted) {
        REQUEST_COUNT += 1;
        true
    } else {
        false
    }
}
fn process(req: Request) -> Result<(), &'static str> {
    let req_arc = Arc::new(Mutex::new(req));
    let req_clone = Arc::clone(&req_arc);
    let handle = thread::spawn(move || {
        let locked = req_clone.lock().unwrap();
        unsafe { check_request(&locked) }
    });
    let valid = handle.join().unwrap();
    if valid {
        Ok(())
    } else {
        Err("Rejected header values")
    }
}
fn main() {
    let raw = "Host: trusted.example.evil.com\nOrigin: http://malicious.example\nReferer: http://malicious.example";
    let request = parse_headers(raw);
    match process(request) {
        Ok(_) => println!("Request processed successfully."),
        Err(err) => println!("Error processing request: {}", err),
    }
}
