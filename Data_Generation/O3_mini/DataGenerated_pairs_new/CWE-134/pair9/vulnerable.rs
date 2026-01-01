/*
This code simulates a scenario where an externally‐controlled format string is used in an unsafe FFI call.
It defines a Handler struct with a method that makes an unsafe call to C’s snprintf using a user‐provided
format string. In addition, concurrency is simulated by running this in a spawned thread.
*/
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::thread;

extern "C" {
    // Declaration for C’s snprintf (varargs FFI)
    fn snprintf(buf: *mut c_char, size: usize, format: *const c_char, ...) -> i32;
}

struct Processor;

impl Processor {
    // This method incorrectly uses the user-controlled string as the format string in snprintf.
    fn process(&self, user_format: &str, value: i32) -> Option<String> {
        unsafe {
            let mut buffer = [0u8; 128];
            // Convert the user_format into a CString. If the conversion fails, return None.
            let c_format = CString::new(user_format).ok()?;
            // FLAW: Passing user_format directly as the formatting string dangerously
            // to snprintf with a single integer argument.
            let ret = snprintf(
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
                c_format.as_ptr(),
                value
            );
            if ret >= 0 {
                let cstr = CStr::from_ptr(buffer.as_ptr() as *const c_char);
                Some(cstr.to_string_lossy().into_owned())
            } else {
                None
            }
        }
    }
}

fn run_parallel(input: String, val: i32) -> Option<String> {
    let proc = Processor;
    // Spawn a thread to emulate concurrent execution
    let handle = thread::spawn(move || {
        proc.process(&input, val)
    });
    handle.join().ok().flatten()
}

fn main() {
    // Malicious input: an externally-controlled format string can lead to unintended reads.
    let user_input = "%x %x %x %x";
    // For demonstration, we supply a single integer value.
    let output = run_parallel(user_input.to_string(), 42).unwrap_or_else(|| "error".to_string());
    println!("Result: {}", output);
}