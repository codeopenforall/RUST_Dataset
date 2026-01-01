/////////////////////////////////////////////////////////////
// Corrected code sample with safe format string usage
/////////////////////////////////////////////////////////////
use std::env;
use std::ffi::{CString, CStr};
use std::sync::{Arc, Mutex};
use std::thread;

extern "C" {
    // libc's snprintf is used safely here.
    fn snprintf(s: *mut i8, n: usize, format: *const i8, ...) -> i32;
}

// This function safely embeds the user input as an argument rather than as the format string.
fn process(user: &str) -> String {
    // Allocate a fixed buffer.
    let mut buffer = [0u8; 256];
    // Convert the user input to a CString.
    let c_user = CString::new(user).expect("Conversion to CString failed");
    // Define a constant format string that safely prints a single string argument.
    let c_fixed_format = CString::new("%s").expect("Conversion to CString failed");
    unsafe {
        // Fixed call: Using a controlled format string and passing user input as a parameter.
        snprintf(
            buffer.as_mut_ptr() as *mut i8,
            buffer.len(),
            c_fixed_format.as_ptr(),
            c_user.as_ptr(),
        );
    }
    // Convert the result in buffer to a Rust String.
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
    c_str.to_string_lossy().into_owned()
}

fn main() {
    // Read input from the command line or default to a benign string.
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "normal_input" };

    // Use Arc and Mutex to simulate concurrent processing.
    let shared_result = Arc::new(Mutex::new(String::new()));
    let mut handles = vec![];

    // Spawn multiple threads that concurrently process the input.
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

    // Wait for all threads to complete.
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    let final_result = shared_result.lock().unwrap();
    println!("Result: {}", *final_result);
}