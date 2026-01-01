//////////////////////////////
// Fixed version code       //
// CWE-134: Issue resolved  //
//////////////////////////////

// Note: Do not include any test or oracle code here.
extern crate libc;

use std::env;
use std::ffi::CString;
use std::thread;

struct Processor;

impl Processor {
    // Executes a safe logging operation by using a constant format string.
    fn run(&self, input: &str) {
        // Convert the user input into a CString.
        let safe_input = CString::new(input).unwrap();
        // Use a fixed format string that treats the user input as data.
        let safe_fmt = CString::new("User input: %s, fixed number: %d\n").unwrap();
        // Spawn a thread to simulate concurrent access.
        let handle = thread::spawn(move || {
            unsafe {
                // Safe: The format string is a constant literal, and the user input is passed as an argument.
                libc::printf(safe_fmt.as_ptr(), safe_input.as_ptr(), 42);
            }
        });
        let _ = handle.join();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <user_input>", args[0]);
        return;
    }
    let input = &args[1];
    let proc_inst = Processor;
    proc_inst.run(input);
}