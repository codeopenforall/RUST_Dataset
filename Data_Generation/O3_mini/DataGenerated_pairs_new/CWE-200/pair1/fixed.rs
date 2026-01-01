///////////////////////////////////////////////////////////////
// Corrected Code Sample: Sensitive Information Protected
///////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Data {
    secret: String,
    value: i32,
}

impl Data {
    fn new(secret: &str, value: i32) -> Self {
        Data {
            secret: secret.to_owned(),
            value,
        }
    }
}

fn process_data(data: &Data, trigger: bool) -> Result<i32, String> {
    // Simulate low-level operations with an unsafe block.
    unsafe {
        // Access the secret using raw pointers (logic preserved).
        let secret_ptr = data.secret.as_ptr();
        let secret_len = data.secret.len();
        let _slice = std::slice::from_raw_parts(secret_ptr, secret_len);
        // On error, only return a generic message without leaking sensitive details.
        if trigger {
            return Err("Processing failed due to an internal error.".to_string());
        }
    }
    Ok(data.value)
}

fn main() {
    // Shared state wrapped inside Arc and Mutex to simulate concurrent access.
    let shared_data = Arc::new(Mutex::new(Data::new("SuperSecret123", 42)));
    let data_for_thread = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        let data = data_for_thread.lock().unwrap();
        // Force triggering the error branch.
        let result = process_data(&data, true);
        if let Err(e) = result {
            // Panic with a generic error message that does not reveal sensitive data.
            panic!("Thread panicked due to: {}", e);
        }
    });
    
    let _ = handle.join();
}