extern crate libc;
use std::env;
use std::ffi::CString;
use std::thread;

// A helper structure that prints messages by invoking an external C function.
struct Printer;

impl Printer {
    // This method dangerously passes the user-provided string directly as the format string
    // to a C printing function without any validation.
    fn output(&self, input: &str) {
        unsafe {
            // UNSAFE USAGE: The following conversion and call allow an attacker to insert
            // format specifiers, leading to potential memory disclosure or corruption.
            let c_format = CString::new(input).expect("Failed to create CString");
            libc::printf(c_format.as_ptr());
        }
    }
}

fn main() {
    // Retrieve external input from command-line arguments.
    let args: Vec<String> = env::args().collect();
    let user_input = if args.len() > 1 {
        args[1].as_str()
    } else {
        "Hello, world!"
    };

    let instance = Printer;
    // Spawn a concurrent thread to simulate multi-threaded usage.
    let handle = thread::spawn(move || {
        instance.output(user_input);
    });
    handle.join().unwrap();
}