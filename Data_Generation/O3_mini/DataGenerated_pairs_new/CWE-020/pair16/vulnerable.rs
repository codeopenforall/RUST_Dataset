///////////////////////////////////////////////
// Vulnerability Example - CWE-020: Improper Input Validation
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
        // For demonstration, write a specific value.
        data[0] = 66; // ASCII 'B'
    }).join().unwrap();

    let mut data_guard = shared_buffer.lock().unwrap();
    
    // -----------------------------------------------------------------
    // Vulnerability: Unsafe pointer arithmetic is used without verifying
    // that the supplied index is within the bounds of the vector.
    unsafe {
        // The next two lines (vulnerability lines) deliberately bypass bounds checking.
        let ptr = data_guard.as_mut_ptr().add(user_index);
        *ptr = 65; // ASCII 'A'
    }
    // -----------------------------------------------------------------

    let result = data_guard.clone();
    // Unsafe conversion from u8 slice to &str assumes valid UTF-8.
    let s = unsafe { std::str::from_utf8_unchecked(&result) };
    Ok(s.to_string())
}

fn main() {
    // Hardcoded input; "15" is out-of-bounds for our 10-element buffer.
    match process_data("15") {
        Ok(result) => println!("Resulting string: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
}