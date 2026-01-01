use std::sync::{Arc, Mutex};
use std::thread;
struct Message {
    headers: Vec<String>,
}
impl Message {
    fn new(raw: &str) -> Self {
        let headers = raw.split("\r\n").map(|s| s.to_string()).collect();
        Message { headers }
    }
    fn extract_length(&self) -> u32 {
        let mut reported: Option<u32> = None;
        for line in &self.headers {
            if let Some(rest) = line.strip_prefix("Content-Length:") {
                let token = rest.trim();
                if let Ok(num) = token.parse::<u32>() {
                    if let Some(existing) = reported {
                        if existing != num {
                            return 0;
                        }
                    } else {
                        reported = Some(num);
                    }
                }
            }
        }
        reported.unwrap_or(0)
    }
}
fn process(data: &str) -> u32 {
    let msg = Message::new(data);
    msg.extract_length()
}
fn simulate(data: &str) -> u32 {
    let shared = Arc::new(Mutex::new(data.to_string()));
    let shared_clone = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        let locked = shared_clone.lock().unwrap();
        process(&locked)
    });
    handler.join().unwrap()
}
fn main() {
    let valid = "POST / HTTP/1.1\r\nHost: secure\r\nContent-Length: 30\r\n\r\n";
    let value = simulate(valid);
    println!("Parsed content length: {}", value);
}
