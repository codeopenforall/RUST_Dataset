use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
struct Request {
    headers: HashMap<String, String>,
}
struct Engine;
impl Engine {
    fn run(&self, req: Arc<Request>) -> bool {
        let shared_req = req.clone();
        let handle = thread::spawn(move || {
            check_header(&shared_req.headers)
        });
        handle.join().unwrap()
    }
}
fn check_header(headers: &HashMap<String, String>) -> bool {
    let anchor = "https://trusted.com";
    if let Some(value) = headers.get("Origin") {
        unsafe {
            let ptr_value = value.as_ptr();
            let ptr_anchor = anchor.as_ptr();
            for i in 0..12 {
                if *ptr_value.add(i) != *ptr_anchor.add(i) {
                    return false;
                }
            }
            true
        }
    } else {
        false
    }
}
fn main() {
    let mut hdrs = HashMap::new();
    hdrs.insert("Origin".to_string(), "https://trusted.com.evil".to_string());
    let request = Arc::new(Request { headers: hdrs });
    let service = Engine;
    let outcome = service.run(request);
    if outcome {
        println!("Request passed validation");
    } else {
        println!("Request failed validation");
    }
}
