//////////////////////////////////////////////////////////////
// A complete Rust program with an externally-controlled format
// vulnerability (CWE-134) via an unsafe call to sprintf.
// Threads concurrently call a function that uses user-supplied
// format string directly with sprintf.
// WARNING: Do not use untrusted input in this manner.
//////////////////////////////////////////////////////////////
use std::env;
use std::ffi::{CString, CStr};
use std::sync::Arc;
use std::thread;

extern "C" {
    fn sprintf(s: *mut i8, format: *const i8, ...) -> i32;
}

// This function takes an input string and uses it as the format
// for sprintf without any validation or fixed formatting string.
pub fn process(input: &str) -> String {
    let mut buffer = [0u8; 128];
    // The user-controlled string is used directly as the format.
    let c_input = CString::new(input).expect("CString::new failed");
    unsafe {
        // Vulnerability: using input as format string (CWE-134).
        sprintf(buffer.as_mut_ptr() as *mut i8, c_input.as_ptr());
    }
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
    c_str.to_string_lossy().into_owned()
}

fn run() {
    let args: Vec<String> = env::args().collect();
    let data = if args.len() > 1 { args[1].clone() } else { String::from("default") };
    let shared = Arc::new(data);
    let mut threads = vec![];
    for _ in 0..5 {
        let data_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            // Each thread calls the process function.
            let _ = process(&data_clone);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().expect("Thread panicked");
    }
}

fn main() {
    run();
}