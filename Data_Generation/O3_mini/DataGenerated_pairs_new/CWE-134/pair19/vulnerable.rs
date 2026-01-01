///////////////////////////////////
// Vulnerability demonstration //
// CWE-134: Externally-Controlled Format String vulnerability
///////////////////////////////////

use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread;

extern "C" {
    // Bind to C's printf function.
    pub fn printf(format: *const i8, ...) -> i32;
}

pub struct Processor {
    // Wraps user input within a smart pointer and mutex to simulate concurrent access.
    data: Arc<Mutex<String>>,
}

impl Processor {
    pub fn new(s: String) -> Self {
        Processor {
            data: Arc::new(Mutex::new(s)),
        }
    }

    pub fn run(&self, input: &str) -> i32 {
        let mut locked = self.data.lock().unwrap();
        *locked = input.to_owned();
        // Convert the current string to a C-style string.
        let c_input = CString::new(locked.clone()).unwrap();
        // FLAW: Using the user-supplied string directly as the format string.
        unsafe {
            // Vulnerability occurs here: the call to printf uses a format string that is externally controlled.
            printf(c_input.as_ptr())
        }
    }
}

// This function uses the processor concurrently in two threads to mimic real-world multi-threaded scenarios.
pub fn process(input: &str) -> i32 {
    let proc_main = Processor::new(String::new());
    // Spawn a concurrent thread.
    let proc_thread = Processor {
        data: Arc::clone(&proc_main.data),
    };
    let input_owned = input.to_owned();
    let handle = thread::spawn(move || {
        proc_thread.run(&input_owned)
    });
    // Run in the main thread as well.
    let main_ret = proc_main.run(input);
    let thread_ret = handle.join().unwrap();
    // Combine results (e.g. number of characters printed) for testing.
    main_ret + thread_ret
}

fn main() {
    // Malicious input containing format specifiers, intended to trigger the vulnerability.
    let malicious = "Hello %x";
    let printed = process(malicious);
    println!("Printed {} characters", printed);
}