/* 
   This Rust program simulates a sensitive data handling scenario.
   It employs concurrency and unsafe pointer dereferencing.
   In the error case (when input != "safe"), it exposes the sensitive secret
   from the stored structure in the error message which is then used in a panic.
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
    // If the input command is not the expected safe string, we trigger an error.
    if input != "safe" {
        // Unsafe block simulating intricate memory access operations.
        let leaked = unsafe {
            let locked = store.lock().unwrap();
            let raw_ptr: *const String = &locked.secret;
            // Dereferencing the raw pointer to clone its value.
            (*raw_ptr).clone()
        };
        // The error message improperly includes the sensitive secret.
        return Err(format!("Access denied. Debug info: secret={}", leaked));
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
                // Panics with an error message that includes the leaked secret.
                panic!("Thread panic: {}", err);
            }
        }
    });
    let _ = handler.join();
}