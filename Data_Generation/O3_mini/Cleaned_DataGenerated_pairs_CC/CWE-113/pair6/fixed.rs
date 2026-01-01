use std::thread;
use std::sync::{Arc, Mutex};
struct HttpResponder;
trait ResponseGenerator {
    fn build_response(&self, input: &str) -> String;
}
impl ResponseGenerator for HttpResponder {
    fn build_response(&self, input: &str) -> String {
        let safe_input: String = input.chars().filter(|&c| c != '\r' && c != '\n').collect();
        let mut header = String::from("HTTP/1.1 200 OK\r\n");
        header.push_str("Content-Length: ");
        header.push_str(&safe_input.len().to_string());
        header.push_str("\r\n\r\n");
        header.push_str(&safe_input);
        header
    }
}
fn assembled_response(input: &'static str) -> String {
    let responder = Arc::new(HttpResponder);
    let result = Arc::new(Mutex::new(String::new()));
    let responder_clone = Arc::clone(&responder);
    let result_clone = Arc::clone(&result);
    let handle = thread::spawn(move || {
        let resp = responder_clone.build_response(input);
        let mut res_lock = result_clone.lock().unwrap();
        *res_lock = resp;
    });
    handle.join().unwrap();
    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}
fn main() {
    let input = "malicious\r\nInjected: Yes";
    let response = assembled_response(input);
    println!("{}", response);
}
