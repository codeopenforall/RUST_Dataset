///////////////////////////////////////////////////////////////
// Vulnerable Code Sample - CWE-134: Externally Controlled Format String
///////////////////////////////////////////////////////////////
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct DataHolder {
    data: i32,
}

impl DataHolder {
    fn new(val: i32) -> Self {
        DataHolder { data: val }
    }
}

// This function blindly uses the incoming string as the format string.
// It converts the string into a slice unsafely, and then passes it to format!.
// That allows an attacker to include extra format specifiers that may cause a panic.
unsafe fn unsafe_format(input: &str, value: i32) -> String {
    let raw = input.as_ptr();
    let slice = std::slice::from_raw_parts(raw, input.len());
    let s = std::str::from_utf8(slice).unwrap();
    // Vulnerability: User provided string s is used as a format string.
    format!(s, value)
}

fn process_input(user_fmt: &str, data: &DataHolder) -> String {
    unsafe { unsafe_format(user_fmt, data.data) }
}

fn main() {
    // Read the format string argument from the command-line.
    let args: Vec<String> = env::args().collect();
    // Default safe simple format if no argument is provided.
    let user_fmt = if args.len() > 1 { &args[1] } else { "{}" };
    let record = DataHolder::new(42);
    let shared_record = Arc::new(Mutex::new(record));
    let mut handles = Vec::new();

    // Spawn multiple threads to mimic a concurrent system.
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared_record);
        // Note: Capturing the user-controlled format string in each thread.
        let fmt_arg = user_fmt.clone();
        let handle = thread::spawn(move || {
            let guard = shared_clone.lock().unwrap();
            let output = process_input(&fmt_arg, &guard);
            println!("{}", output);
        });
        handles.push(handle);
    }
    for handle in handles {
        // Join the threads and propagate any panics.
        handle.join().unwrap();
    }
}