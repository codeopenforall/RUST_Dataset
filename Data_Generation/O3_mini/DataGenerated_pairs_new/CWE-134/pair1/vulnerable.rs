use std::io::{self, Write};
extern crate libc;
use std::ffi::CString;

pub struct Handler;

impl Handler {
    pub fn new() -> Self {
        Handler
    }

    // This method takes user data and uses it directly as a format string.
    pub fn process(&self, payload: &str) -> String {
        // Allocate a fixed-size buffer
        let mut buffer = [0u8; 256];
        // Create a C string from user input; note that this can fail if payload contains a null byte.
        let fmt = CString::new(payload).expect("Failed to create CString from payload");

        unsafe {
            // Vulnerability (CWE-134): The user-controlled string is used as the format specifier
            // in the sprintf call. If payload contains format specifiers, this may lead to arbitrary
            // memory reads and/or other unexpected behavior.
            libc::sprintf(buffer.as_mut_ptr() as *mut i8, fmt.as_ptr());
        }
        // Convert the buffer (up to the first null byte) into a Rust String.
        let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        String::from_utf8_lossy(&buffer[..len]).into_owned()
    }
}

fn main() {
    let handler = Handler::new();
    // Simulate user-supplied payload which contains malicious format specifiers.
    let input = "User data: %x %x";
    let output = handler.process(input);
    println!("{}", output);
}