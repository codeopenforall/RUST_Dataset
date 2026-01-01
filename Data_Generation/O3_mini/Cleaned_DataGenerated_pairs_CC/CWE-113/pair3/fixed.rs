use std::sync::{Arc, Mutex};
use std::thread;
struct ResponseBuilder {
    headers: Vec<String>,
}
impl ResponseBuilder {
    fn new() -> Self {
        ResponseBuilder { headers: Vec::new() }
    }
    fn add_header(&mut self, key: &str, value: &str) {
        let safe_value: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
        self.headers.push(format!("{}: {}", key, safe_value));
    }
    fn build(&self) -> String {
        let mut result = String::new();
        for header in &self.headers {
            result.push_str(header);
            result.push_str("\r\n");
        }
        result.push_str("\r\n");
        result
    }
}
pub fn process(input: &str) -> String {
    let builder = Arc::new(Mutex::new(ResponseBuilder::new()));
    let safe_input = input.to_owned();
    let clone_builder = builder.clone();
    let handle = thread::spawn(move || {
        let mut rb = clone_builder.lock().unwrap();
        rb.add_header("X-Custom", &safe_input);
    });
    handle.join().unwrap();
    let rb = builder.lock().unwrap();
    rb.build()
}
fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "default".to_string());
    let response = process(&arg);
    println!("{}", response);
}
