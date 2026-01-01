///////////////////////////////////////////////
// Corrected Example - Secure Input Validation
///////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

fn process_data(input: &str) -> Result<String, &'static str> {
    // Parse the user-supplied index from input.
    let user_index: usize = input.trim().parse().map_err(|_| "Invalid number")?;
    
    // Create a shared buffer of fixed length.
    let shared_buffer = Arc::new(Mutex::new(vec![0u8; 10]));
    let thread_buffer = Arc::clone(&shared_buffer);

    // Spawn a concurrent thread that modifies the buffer.
    thread::spawn(move || {
        let mut data = thread_buffer.lock().unwrap();
        data[0] = 66; // ASCII 'B'
    }).join().unwrap();

    let mut data_guard = shared_buffer.lock().unwrap();

    // -----------------------------------------------------------------
    // Fix applied: Validate the user-supplied index against the buffer length.
    if user_index >= data_guard.len() {
         return Err("Index out-of-bounds");
    }
    // Use safe array indexing after validation.
    data_guard[user_index] = 65; // ASCII 'A'
    // -----------------------------------------------------------------

    let result = data_guard.clone();
    // Perform a checked conversion from u8 slice to &str.
    let s = std::str::from_utf8(&result).map_err(|_| "UTF-8 conversion error")?;
    Ok(s.to_string())
}

fn main() {
    // Hardcoded input; "15" will now be rejected safely.
    match process_data("15") {
        Ok(result) => println!("Resulting string: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
}