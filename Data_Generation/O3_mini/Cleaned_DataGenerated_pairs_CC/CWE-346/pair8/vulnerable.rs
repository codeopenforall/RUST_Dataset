use std::collections::HashMap;
use std::thread;
use std::time::Duration;
struct Request {
    headers: HashMap<String, String>,
}
impl Request {
    fn new() -> Self {
        Request {
            headers: HashMap::new(),
        }
    }
}
unsafe fn check_origin(req: &Request) -> bool {
    if let Some(val) = req.headers.get("host") {
        if std::ptr::eq(val.as_str(), "trusted.example.com") {
            return true;
        }
    }
    false
}
fn process_request(req: &Request) -> bool {
    unsafe { check_origin(req) }
}
fn main() {
    let req = {
        let mut r = Request::new();
        r.headers.insert("host".to_string(), String::from("trusted.example.com"));
        r
    };
    let handler = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let valid = process_request(&req);
        if valid {
            println!("Access granted.");
        } else {
            println!("Access denied.");
        }
        valid
    });
    let res = handler.join().unwrap();
    if res {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
