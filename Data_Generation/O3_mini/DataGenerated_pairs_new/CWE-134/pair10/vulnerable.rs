/////////////////////////////////////////////////////////////
// Vulnerable code sample for externally-controlled format string
/////////////////////////////////////////////////////////////
use std::env;
use std::ffi::{CString, CStr};
use std::sync::{Arc, Mutex};
use std::thread;

extern "C" {
    // libc's snprintf provides a variadic interface. When an attacker controls
    // the format string, format specifiers (like %x, %n, etc.) can cause undefined behavior.
    fn snprintf(s: *mut i8, n: usize, format: *const i8, ...) -> i32;
}

// This function unsafely passes user input as the format string to snprintf.
// If the input includes format specifiers, it may trigger a vulnerability.
fn process(user: &str) -> String {
    // Allocate a fixed-size buffer.
    let mut buffer = [0u8; 256];
    // Convert the user input directly to a CString.
    // No validation or sanitization is applied.
    let c_format = CString::new(user).expect("Conversion to CString failed");
    unsafe {
        // Vulnerable call: Using an externally controlled format string.
        // If user contains format specifiers, snprintf will try to access missing arguments.
        snprintf(
            buffer.as_mut_ptr() as *mut i8,
            buffer.len(),
            c_format.as_ptr(),
        );
    }
    // Convert the C-style string in buffer to a Rust String.
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
    c_str.to_string_lossy().into_owned()
}

fn main() {
    // Read input from command line or use a default.
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "normal_input" };

    // Use Arc and Mutex to simulate concurrent processing.
    let shared_result = Arc::new(Mutex::new(String::new()));
    let mut handles = vec![];

    // Spawn several threads that concurrently process the input.
    for _ in 0..4 {
        let input_clone = input.to_string();
        let shared_clone = Arc::clone(&shared_result);
        let handle = thread::spawn(move || {
            let res = process(&input_clone);
            let mut data = shared_clone.lock().unwrap();
            *data = res;
        });
        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    let final_result = shared_result.lock().unwrap();
    println!("Result: {}", *final_result);
}