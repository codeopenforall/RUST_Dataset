//////////////////////////////////////////////////////////////
// A complete Rust program with the vulnerability fixed.
// Instead of using the user-controlled string as the format specifier,
// a constant format string ("%s") is used, which treats the user input 
// strictly as data. This protects against format string attacks.
//////////////////////////////////////////////////////////////
use std::env;
use std::ffi::{CString, CStr};
use std::sync::Arc;
use std::thread;

extern "C" {
    fn sprintf(s: *mut i8, format: *const i8, ...) -> i32;
}

// This function safely prints the input by using a constant format string.
pub fn process(input: &str) -> String {
    let mut buffer = [0u8; 128];
    // Use a fixed format string to avoid interpreting the input as formatting directives.
    let c_format = CString::new("%s").expect("CString::new failed");
    let c_input = CString::new(input).expect("CString::new failed");
    unsafe {
        sprintf(
            buffer.as_mut_ptr() as *mut i8,
            c_format.as_ptr(),
            c_input.as_ptr(),
        );
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