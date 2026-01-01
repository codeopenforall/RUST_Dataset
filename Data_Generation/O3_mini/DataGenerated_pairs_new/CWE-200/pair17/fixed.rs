/* 
   This Rust program maintains the same logical workflow as before,
   with concurrency and similar structure, but it fixes the vulnerability
   by not including sensitive data in any error or log messages.
*/
use std::sync::{Arc, Mutex};
use std::thread;
use std::panic;

struct Data {
    secret: String,
}

impl Data {
    fn new(secret: &str) -> Self {
        Data { secret: secret.to_string() }
    }
}

fn process(input: &str, store: &Arc<Mutex<Data>>) -> Result<String, String> {
    // Check input without exposing any data on error.
    if input != "safe" {
        // Removed unsafe secret extraction and only returns a generic error message.
        return Err("Access denied.".to_string());
    }
    Ok("Operation successful".to_string())
}

fn main() {
    let store = Arc::new(Mutex::new(Data::new("S3cr3tP@ss")));
    let arg = std::env::args().nth(1).unwrap_or_else(|| "unsafe".to_string());
    let store_clone = Arc::clone(&store);
    let handler = thread::spawn(move || {
        match process(&arg, &store_clone) {
            Ok(message) => println!("{}", message),
            Err(err) => {
                // Panics with a generic error message, avoiding any leakage of sensitive data.
                panic!("Thread panic: {}", err);
            }
        }
    });
    let _ = handler.join();
}