//////////////////////////////
// Vulnerable version code  //
// CWE-134: Externally-Controlled Format String vulnerability
//////////////////////////////

// Note: Do not include any test or oracle code here.
extern crate libc;

use std::env;
use std::ffi::CString;
use std::thread;

struct Processor;

impl Processor {
    // Executes a potentially dangerous formatting operation by using user-controlled input as the format string.
    fn run(&self, fmt: &str) {
        let fmt_owned = fmt.to_owned();
        // Spawn a thread to simulate concurrent access
        let handle = thread::spawn(move || {
            // Convert the user provided format string into a CString.
            // This user input is not sanitized and is used directly in the unsafe call.
            let cstr = CString::new(fmt_owned).unwrap();
            unsafe {
                // Vulnerable: The user-controlled string is used as the format string.
                // This can allow format string attacks if the string contains format specifiers.
                libc::printf(cstr.as_ptr(), 42);
            }
        });
        let _ = handle.join();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <format_string>", args[0]);
        return;
    }
    let fmt = &args[1];
    let proc_inst = Processor;
    proc_inst.run(fmt);
}