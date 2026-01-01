/*
This program simulates a format string vulnerability.
It reads a user-controlled string from command line, spawns a thread,
and then passes the user input directly as the format string to libc::snprintf.
The unsafe block converts the Rust string into a C string and calls libc::snprintf without any
format argument substitutions.  If the input contains format specifiers (e.g. “%s”, “%x”), then
undefined behavior may occur, exposing memory information or causing a crash.
*/
use std::env;
use std::ffi::{CString, CStr};
use std::ptr;
use std::sync::Arc;
use std::thread;

extern "C" {
    fn snprintf(buf: *mut i8, size: usize, format: *const i8, ...) -> i32;
}

fn process(input: &str) -> String {
    // Directly use the externally provided format string.
    let c_format = CString::new(input).expect("CString conversion failed");
    let mut buffer = [0i8; 256];
    // Unsafe call: passing user-controlled string as the format string.
    unsafe {
        // Vulnerable: using user input as the formatting string
        let _ = snprintf(buffer.as_mut_ptr(), buffer.len(), c_format.as_ptr());
        CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned()
    }
}

fn run_app() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        return;
    }
    let user_input = args[1].clone();
    let shared = Arc::new(user_input);
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || process(&shared_clone));
    let result = handle.join().unwrap();
    println!("{}", result);
}

fn main() {
    run_app();
}