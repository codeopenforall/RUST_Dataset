use std::collections::HashMap;
use std::thread;
struct Request {
    headers: HashMap<String, String>,
}
impl Request {
    fn new(origin: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("Origin"), origin.to_string());
        Request { headers }
    }
}
fn validate(req: &Request) -> bool {
    let trusted = "https://valid.example.com";
    if let Some(candidate) = req.headers.get("Origin") {
        unsafe {
            let cand_ptr = candidate.as_ptr();
            let trusted_ptr = trusted.as_ptr();
            if cand_ptr == trusted_ptr {
                return true;
            }
        }
    }
    false
}
fn run() {
    let req = Request::new("https://valid.example.com");
    if validate(&req) {
        println!("Access granted.");
    } else {
        println!("Access denied.");
    }
}
fn main() {
    let handle = thread::spawn(|| {
        run();
    });
    handle.join().unwrap();
}
