//////////////////////////////////////////////
// Vulnerable Code Sample - Externally-Controlled Format String
//////////////////////////////////////////////
extern crate libc;

use std::cell::UnsafeCell;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
use std::sync::Arc;
use std::thread;
use std::env;

// A shared buffer that is concurrently accessed.
pub struct SharedBuffer {
    buf: UnsafeCell<[u8; 128]>,
}

// Mark as Sync since we guarantee external synchronization.
unsafe impl Sync for SharedBuffer {}

impl SharedBuffer {
    pub fn new() -> Self {
        SharedBuffer { buf: UnsafeCell::new([0; 128]) }
    }

    // Function using unsafe FFI call to sprintf.
    // CWE-134: The format string is externally controlled (user_input)
    pub fn write(&self, user_input: &str, value: i32) -> Result<(), ()> {
        // Convert user input into a C string. Malformed input is rejected.
        let c_fmt = CString::new(user_input).map_err(|_| ())?;
        unsafe {
            // POTENTIAL FLAW (CWE-134): Passing an externally-controlled format string to sprintf.
            // This may lead to unexpected behavior if user_input contains format specifiers.
            libc::sprintf(self.buf.get() as *mut c_char, c_fmt.as_ptr(), value);
        }
        Ok(())
    }

    pub fn get(&self) -> String {
        unsafe {
            // Read the C string from the internal buffer.
            let c_str = CStr::from_ptr(self.buf.get() as *const c_char);
            c_str.to_string_lossy().into_owned()
        }
    }
}

// Public API that processes user input and returns the formatted output.
// This function is used by the test oracle.
#[no_mangle]
pub fn process_input(user_input: &str) -> String {
    let shared = Arc::new(SharedBuffer::new());
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        // Note: value 42 is used as the argument to be formatted.
        let _ = shared_clone.write(user_input, 42);
    });
    handle.join().expect("Thread panicked");
    shared.get()
}

fn main() {
    // Read user input from command line arguments.
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "Default: %d" };
    let result = process_input(input);
    println!("Result: {}", result);
}