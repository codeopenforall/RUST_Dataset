use std::sync::{Arc, Mutex};
use std::thread;
struct Response {
    header: String,
}
impl Response {
    fn new(input: &str) -> Self {
        let head = build_header(input);
        Response { header: head }
    }
}
fn build_header(user_data: &str) -> String {
    let safe_input: String = user_data.chars()
        .filter(|&c| c != '\r' && c != '\n')
        .collect();
    let prefix = "X-Custom: ";
    format!("{}{}", prefix, safe_input)
}
fn main() {
    let shared_resp = Arc::new(Mutex::new(None::<Response>));
    let cloned_resp = Arc::clone(&shared_resp);
    let input_data = "user-input\r\nInjected: malicious-header"; 
    let handler = thread::spawn(move || {
        let resp = Response::new(input_data);
        let mut lock = cloned_resp.lock().unwrap();
        *lock = Some(resp);
    });
    handler.join().unwrap();
    let lock = shared_resp.lock().unwrap();
    if let Some(resp) = &*lock {
        println!("Created header: {}", resp.header);
    }
}
