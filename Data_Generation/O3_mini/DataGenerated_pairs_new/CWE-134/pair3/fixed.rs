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
        // Lock shared state to simulate concurrent logging.
        let mut stored = self.data.lock().unwrap();
        *stored = input.to_owned();
        // FIX: Use a constant format string that safely prints the user input.
        let safe_format = CString::new("%s").unwrap();
        let user_value = CString::new(stored.as_str()).unwrap();
        unsafe {
            // The fixed version supplies a benign constant format string ("%s")
            // and passes the user-controlled string as a parameter.
            printf(safe_format.as_ptr(), user_value.as_ptr());
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