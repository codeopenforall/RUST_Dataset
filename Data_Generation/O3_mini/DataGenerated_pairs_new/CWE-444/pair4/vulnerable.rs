use std::str;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

struct Message {
    data: String,
}

impl Message {
    fn new(data: String) -> Self {
        Message { data }
    }

    // This function unsafely tries to locate and parse the "Content-Length:" header
    // by scanning the raw bytes and using unchecked UTF-8 conversion.
    // It returns the first numerical value found even if multiple headers are present.
    fn extract_length(&self) -> Option<u32> {
        let bytes = self.data.as_bytes();
        unsafe {
            let base = bytes.as_ptr();
            let total = bytes.len();
            let mut i = 0;
            while i < total {
                // Check if the upcoming bytes match "Content-Length:" using unchecked conversion.
                if i + 15 < total
                    && str::from_utf8_unchecked(std::slice::from_raw_parts(base.add(i), 15))
                        == "Content-Length:"
                {
                    // Directly jump past header bytes to interpret the remaining slice as the value.
                    let num_ptr = base.add(i + 15);
                    let num_slice = std::slice::from_raw_parts(num_ptr, total - i - 15);
                    if let Ok(s) = str::from_utf8(num_slice) {
                        let token = s.trim().split_whitespace().next().unwrap_or("");
                        if let Ok(val) = token.parse::<u32>() {
                            // Vulnerability: returns first parsed value even if duplicate headers exist.
                            return Some(val);
                        }
                    }
                }
                i += 1;
            }
        }
        None
    }
}

fn process(data: &str) -> u32 {
    let msg = Message::new(data.to_string());
    msg.extract_length().unwrap_or(0)
}

fn simulate(data: &str) -> u32 {
    // Simulate concurrent access to the request data.
    let shared = Arc::new(Mutex::new(data.to_string()));
    let shared_clone = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        let locked = shared_clone.lock().unwrap();
        process(&locked)
    });
    handler.join().unwrap()
}

fn main() {
    // In a real-world scenario the request might be forwarded between proxies and backends.
    // An attacker can craft a payload with multiple Content-Length headers.
    let crafted = "POST / HTTP/1.1\r\nHost: vulnerable\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n";
    let value = simulate(crafted);
    println!("Parsed content length: {}", value);
}