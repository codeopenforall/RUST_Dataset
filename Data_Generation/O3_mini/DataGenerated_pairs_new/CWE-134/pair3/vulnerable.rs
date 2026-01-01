use std::env;
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread;

extern "C" {
    fn printf(format: *const i8, ...) -> i32;
}

struct Logger {
    data: Mutex<String>,
}

impl Logger {
    fn new(s: &str) -> Self {
        Logger {
            data: Mutex::new(s.to_owned()),
        }
    }

    fn record(&self, input: &str) {
        // Locking shared state to simulate concurrent logging.
        let mut stored = self.data.lock().unwrap();
        *stored = input.to_owned();
        // Create a C-style string directly from the (potentially malicious) user input.
        let fmt_str = CString::new(stored.as_str()).unwrap();
        unsafe {
            // Vulnerability: The user-controlled string is used directly as a format string.
            // If the input contains format specifiers (e.g. "%n"), this call can lead to undefined behavior.
            printf(fmt_str.as_ptr());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = "No format provided";
    let user_in = if args.len() > 1 { &args[1] } else { default };

    let logger = Arc::new(Logger::new("Initial"));
    let logger_clone = Arc::clone(&logger);

    let handle = thread::spawn(move || {
        logger_clone.record(user_in);
    });
    handle.join().unwrap();
}