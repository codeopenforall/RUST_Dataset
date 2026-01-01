//////////////////////////
// Vulnerable Code Sample
//////////////////////////
use std::env;
use std::ffi::CString;
use std::thread;
extern crate libc;

struct Printer;

impl Printer {
    // This method takes a user-controlled string and directly
    // passes it as the format string to libc::printf, making it
    // vulnerable to externally-controlled format string attacks.
    fn output(&self, input: &str) {
        // Create a C-compatible string without further validation.
        let raw = CString::new(input).unwrap();
        unsafe {
            // Unsafe block: The user input is used as the format string.
            // This can cause an exploit when format specifiers are injected.
            libc::printf(raw.as_ptr());
        }
    }
}

fn execute() {
    let args: Vec<String> = env::args().collect();
    // If no argument is provided, use a default string that contains a format specifier.
    // An attacker could supply something like "test %s" to trigger the vulnerability.
    let user_str = if args.len() > 1 { &args[1] } else { "Hello, %s!" };
    let printer = Printer;
    // Spawn a thread to simulate concurrent use.
    let handle = thread::spawn(move || {
        printer.output(user_str);
    });
    handle.join().unwrap();
}

fn main() {
    execute();
}